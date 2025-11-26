use axum::http::StatusCode;
use axum::response::IntoResponse;
use domain::DomainError;
use http_problem::ProblemDetails;

/// AppError is a newtype wrapper around anyhow::Error that implements IntoResponse
/// to return RFC9457-compliant Problem Details JSON responses.
///
/// This error type follows the pattern described in:
/// https://rup12.net/posts/learning-rust-custom-errors/
///
/// It allows using the `?` operator throughout handler functions, automatically
/// converting various error types into appropriate HTTP responses.
///
/// # Example
/// ```ignore
/// async fn handler(State(state): State<AppState>) -> AppResult<Json<Response>> {
///     let user = User::find_by_id(&state.db, id).await?; // DomainError -> AppError
///     let data = process(user)?; // anyhow::Error -> AppError
///     Ok(Json(data))
/// }
/// ```
#[derive(Debug)]
pub struct AppError(anyhow::Error);

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self(anyhow::anyhow!(message.into()))
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self(anyhow::anyhow!(message.into()))
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self(anyhow::anyhow!(message.into()))
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self(anyhow::anyhow!(message.into()))
    }

    #[allow(dead_code)]
    pub fn internal(message: impl Into<String>) -> Self {
        Self(anyhow::anyhow!(message.into()))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_msg = self.0.to_string();
        
        let (status, title) = if let Some(domain_err) = self.0.downcast_ref::<DomainError>() {
            match domain_err {
                DomainError::Conflict { .. } => (StatusCode::CONFLICT, "Conflict"),
                DomainError::NotFound { .. } => (StatusCode::NOT_FOUND, "Not Found"),
                DomainError::UnauthorizedAction => (StatusCode::UNAUTHORIZED, "Unauthorized"),
                _ => (StatusCode::UNPROCESSABLE_ENTITY, "Validation Error"),
            }
        } else if error_msg.contains("already registered") || error_msg.contains("already exists") {
            (StatusCode::CONFLICT, "Conflict")
        } else if error_msg.contains("not found") {
            (StatusCode::NOT_FOUND, "Not Found")
        } else if error_msg.contains("invalid credentials") || error_msg.contains("unauthorized") {
            (StatusCode::UNAUTHORIZED, "Unauthorized")
        } else if error_msg.contains("cannot follow") || error_msg.contains("validation") {
            (StatusCode::UNPROCESSABLE_ENTITY, "Validation Error")
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        };
        
        ProblemDetails::new(status)
            .with_title(title)
            .with_detail(error_msg)
            .into_response()
    }
}

// Legacy type aliases for backward compatibility
pub type ApiError = AppError;
pub type ApiResult<T> = AppResult<T>;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_app_error_validation() {
        let err = AppError::validation("Invalid input");
        assert_eq!(err.0.to_string(), "Invalid input");
    }

    #[test]
    fn test_app_error_conflict() {
        let err = AppError::conflict("Resource conflict");
        assert_eq!(err.0.to_string(), "Resource conflict");
    }

    #[test]
    fn test_app_error_unauthorized() {
        let err = AppError::unauthorized("Not authorized");
        assert_eq!(err.0.to_string(), "Not authorized");
    }

    #[test]
    fn test_app_error_not_found() {
        let err = AppError::not_found("Resource not found");
        assert_eq!(err.0.to_string(), "Resource not found");
    }

    #[test]
    fn test_app_error_internal() {
        let err = AppError::internal("Internal error");
        assert_eq!(err.0.to_string(), "Internal error");
    }

    #[test]
    fn test_app_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("Test error");
        let app_err: AppError = anyhow_err.into();
        assert_eq!(app_err.0.to_string(), "Test error");
    }

    #[test]
    fn test_app_error_from_domain_conflict() {
        let domain_err = DomainError::Conflict {
            entity: "user",
        };
        let app_err: AppError = domain_err.into();
        let response = app_err.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_app_error_from_domain_not_found() {
        let domain_err = DomainError::NotFound {
            entity: "user",
        };
        let app_err: AppError = domain_err.into();
        let response = app_err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_from_domain_unauthorized() {
        let domain_err = DomainError::UnauthorizedAction;
        let app_err: AppError = domain_err.into();
        let response = app_err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_app_error_message_contains_already_registered() {
        let err = AppError::conflict("email already registered");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_app_error_message_contains_not_found() {
        let err = AppError::validation("user not found");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_message_contains_invalid_credentials() {
        let err = AppError::validation("invalid credentials");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_app_error_message_contains_validation() {
        let err = AppError::validation("validation error");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_app_error_generic_error() {
        let err = AppError::from(anyhow::anyhow!("Some random error"));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_message_contains_cannot_follow() {
        let err = AppError::validation("cannot follow yourself");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_app_error_message_contains_already_exists() {
        let err = AppError::from(anyhow::anyhow!("resource already exists"));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
