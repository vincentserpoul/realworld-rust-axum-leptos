use axum::{Json, http::StatusCode, response::IntoResponse};
use domain::DomainError;
use serde::Serialize;

type Messages = Vec<String>;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    messages: Messages,
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Serialize)]
struct ErrorEnvelope {
    errors: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    body: Messages,
}

impl ApiError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, vec![message.into()])
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNPROCESSABLE_ENTITY, vec![message.into()])
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, vec![message.into()])
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, vec![message.into()])
    }

    #[allow(dead_code)]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, vec![message.into()])
    }

    fn new(status: StatusCode, messages: Messages) -> Self {
        Self { status, messages }
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::Conflict { .. } => Self::conflict(err.to_string()),
            DomainError::NotFound { .. } => Self::not_found(err.to_string()),
            DomainError::UnauthorizedAction => Self::unauthorized(err.to_string()),
            _ => Self::validation(err.to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = ErrorEnvelope {
            errors: ErrorBody {
                body: self.messages,
            },
        };
        (self.status, Json(body)).into_response()
    }
}
