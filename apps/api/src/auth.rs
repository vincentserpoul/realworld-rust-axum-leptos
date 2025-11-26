use axum::{
    extract::{FromRequestParts, OptionalFromRequest, Request},
    http::{header::AUTHORIZATION, request::Parts},
};
use domain::{
    AuthToken, User,
    repositories::UsersRepository,
};

use crate::{error::ApiError, state::AppState};

#[derive(Clone)]
pub struct CurrentUser {
    pub user: User,
    pub token: AuthToken,
}

impl<U, A, C> FromRequestParts<AppState<U, A, C>> for CurrentUser
where
    U: UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<U, A, C>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let token_header = parts.headers.get(AUTHORIZATION).cloned();
        let state = state.clone();
        async move {
            let header = token_header
                .ok_or_else(|| ApiError::unauthorized("missing authorization header"))?;
            let header = header
                .to_str()
                .map_err(|_| ApiError::unauthorized("invalid authorization header"))?;
            let token_value = header
                .strip_prefix("Token ")
                .or_else(|| header.strip_prefix("Bearer "))
                .unwrap_or(header)
                .trim();
            if token_value.is_empty() {
                return Err(ApiError::unauthorized("invalid authorization header"));
            }

            let sessions = state.sessions.read().await;
            let user_id = sessions
                .get(token_value)
                .copied()
                .ok_or_else(|| ApiError::unauthorized("invalid token"))?;
            drop(sessions);

            let user = state.use_cases.users_repo
                .get_user_by_id(user_id)
                .await
                .map_err(|_| ApiError::internal("database error"))?
                .ok_or_else(|| ApiError::not_found("user"))?;

            let token = AuthToken::new(token_value.to_owned()).map_err(ApiError::from)?;
            Ok(Self { user, token })
        }
    }
}

// Use blanket FromRequest implementation provided by axum_core via FromRequestParts.

impl<U, A, C> OptionalFromRequest<AppState<U, A, C>> for CurrentUser
where
    U: UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    type Rejection = ApiError;

    fn from_request(
        request: Request,
        state: &AppState<U, A, C>,
    ) -> impl std::future::Future<Output = Result<Option<Self>, Self::Rejection>> + Send {
        let state = state.clone();
        async move {
            let (mut parts, _) = request.into_parts();
            match <Self as FromRequestParts<AppState<U, A, C>>>::from_request_parts(&mut parts, &state)
                .await
            {
                Ok(user) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::repositories::UsersRepository;
    use axum::http::Request;
    use domain::{Email, PasswordHash, UserId, Username};

    fn create_test_user(id: UserId, username: &str, email: &str) -> User {
        User::new(
            id,
            Email::parse(email).unwrap(),
            Username::new(username).unwrap(),
            PasswordHash::new("hash".to_string()).unwrap(),
            chrono::Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_missing_header() {
        let state = AppState::default();
        let req = Request::builder()
            .uri("/")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_invalid_header() {
        let state = AppState::default();
        let invalid_bytes = vec![0x80, 0x81];
        let req = Request::builder()
            .uri("/")
            .header("authorization", invalid_bytes)
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_empty_token() {
        let state = AppState::default();
        let req = Request::builder()
            .uri("/")
            .header("authorization", "Token ")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_invalid_token() {
        let state = AppState::default();
        let req = Request::builder()
            .uri("/")
            .header("authorization", "Token invalid-token")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_success_with_token_prefix() {
        let state = AppState::default();
        let user_id = UserId::random();
        let token = AuthToken::new("valid-token".to_string()).unwrap();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let req = Request::builder()
            .uri("/")
            .header("authorization", "Token valid-token")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_ok());
        let current_user = result.unwrap();
        assert_eq!(current_user.user.id, user_id);
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_success_with_bearer_prefix() {
        let state = AppState::default();
        let user_id = UserId::random();
        let token = AuthToken::new("valid-token".to_string()).unwrap();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let req = Request::builder()
            .uri("/")
            .header("authorization", "Bearer valid-token")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_ok());
        let current_user = result.unwrap();
        assert_eq!(current_user.user.id, user_id);
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_success_without_prefix() {
        let state = AppState::default();
        let user_id = UserId::random();
        let token = AuthToken::new("valid-token".to_string()).unwrap();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let req = Request::builder()
            .uri("/")
            .header("authorization", "valid-token")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_current_user_from_request_parts_user_not_found() {
        let state = AppState::default();
        let user_id = UserId::random();
        let token = AuthToken::new("valid-token".to_string()).unwrap();
        
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let req = Request::builder()
            .uri("/")
            .header("authorization", "Token valid-token")
            .body(())
            .unwrap();
        
        let (mut parts, _) = req.into_parts();
        let result = CurrentUser::from_request_parts(&mut parts, &state).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_optional_from_request_returns_none_on_error() {
        let state = AppState::default();
        let req = Request::builder()
            .uri("/")
            .body(axum::body::Body::empty())
            .unwrap();
        
        let result = CurrentUser::from_request(req, &state).await;
        
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_optional_from_request_returns_some_on_success() {
        let state = AppState::default();
        let user_id = UserId::random();
        let token = AuthToken::new("valid-token".to_string()).unwrap();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let req = Request::builder()
            .uri("/")
            .header("authorization", "Token valid-token")
            .body(axum::body::Body::empty())
            .unwrap();
        
        let result = CurrentUser::from_request(req, &state).await;
        
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}

