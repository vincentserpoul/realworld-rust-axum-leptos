use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::Utc;
use domain::{
    AuthToken, PlainPassword, UserEnvelope,
    use_cases::{
        login_user, register_user,
        LoginUserInput as LoginInput, RegisterUserInput as RegisterInput,
    },
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult},
    state::AppState,
};

pub fn router<U, A, C>() -> Router<AppState<U, A, C>>
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new()
        .route("/", post(register_user_handler))
        .route("/login", post(login_user_handler))
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    user: RegisterPayload,
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    user: LoginPayload,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

async fn register_user_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<impl IntoResponse>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let password = PlainPassword::new(req.user.password)?;
    let password_hash = hash_password(&password)?;

    let input = RegisterInput {
        username: req.user.username,
        email: req.user.email,
        password_hash,
    };

    let output = register_user(&state.use_cases.users_repo, input, Utc::now())
        .await
        .map_err(|e| match e {
            domain::DomainError::Conflict { entity: "email" } => {
                ApiError::conflict("email already registered")
            }
            domain::DomainError::Conflict { entity: "username" } => {
                ApiError::conflict("username already taken")
            }
            _ => ApiError::from(e),
        })?;

    let token = issue_token()?;
    remember_session(&state, &token, output.user.id).await;

    let view = output.user.to_view(Some(token));
    Ok((StatusCode::CREATED, Json(UserEnvelope::from(view))))
}

async fn login_user_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<impl IntoResponse>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let password = PlainPassword::new(req.user.password)?;
    let password_hash = hash_password(&password)?;

    let input = LoginInput {
        email: req.user.email,
        password_hash,
    };

    let output = login_user(&state.use_cases.users_repo, input)
        .await
        .map_err(|_| ApiError::unauthorized("invalid credentials"))?;

    let token = issue_token()?;
    remember_session(&state, &token, output.user.id).await;

    let view = output.user.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

pub(crate) fn hash_password(password: &PlainPassword) -> ApiResult<domain::PasswordHash> {
    let salted = format!("hashed:{}", password.as_str());
    domain::PasswordHash::new(salted).map_err(ApiError::from)
}

fn issue_token() -> ApiResult<AuthToken> {
    AuthToken::new(Uuid::new_v4().to_string()).map_err(ApiError::from)
}

async fn remember_session<U, A, C>(state: &AppState<U, A, C>, token: &AuthToken, user_id: domain::UserId)
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    state
        .sessions
        .write()
        .await
        .insert(token.as_str().to_owned(), user_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use domain::repositories::UsersRepository;
    use domain::{Email, PasswordHash, User, UserId, Username};
    use tower::ServiceExt;

    #[test]
    fn test_hash_password() {
        let password = PlainPassword::new("password123").unwrap();
        let hash = hash_password(&password).unwrap();
        assert!(hash.as_str().starts_with("hashed:"));
        assert!(hash.as_str().contains("password123"));
    }

    #[test]
    fn test_issue_token() {
        let token1 = issue_token().unwrap();
        let token2 = issue_token().unwrap();
        assert_ne!(token1.as_str(), token2.as_str());
    }

    #[tokio::test]
    async fn test_remember_session() {
        let state = AppState::default();
        let user_id = UserId::random();
        let token = AuthToken::new("test-token".to_string()).unwrap();
        
        remember_session(&state, &token, user_id).await;
        
        let sessions = state.sessions.read().await;
        assert_eq!(sessions.get("test-token"), Some(&user_id));
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let state = AppState::default();
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "username": "testuser",
                "email": "test@example.com",
                "password": "password123"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_register_user_duplicate_email() {
        let state = AppState::default();
        
        let user = User::new(
            UserId::random(),
            Email::parse("test@example.com").unwrap(),
            Username::new("existing").unwrap(),
            PasswordHash::new("hash".to_string()).unwrap(),
            chrono::Utc::now(),
        );
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "username": "newuser",
                "email": "test@example.com",
                "password": "password123"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_login_user_success() {
        let state = AppState::default();
        let password = PlainPassword::new("password123").unwrap();
        let password_hash = hash_password(&password).unwrap();
        
        let user = User::new(
            UserId::random(),
            Email::parse("test@example.com").unwrap(),
            Username::new("testuser").unwrap(),
            password_hash,
            chrono::Utc::now(),
        );
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "email": "test@example.com",
                "password": "password123"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_user_invalid_email() {
        let state = AppState::default();
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "email": "nonexistent@example.com",
                "password": "password123"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_login_user_invalid_password() {
        let state = AppState::default();
        let password = PlainPassword::new("correctpassword").unwrap();
        let password_hash = hash_password(&password).unwrap();
        
        let user = User::new(
            UserId::random(),
            Email::parse("test@example.com").unwrap(),
            Username::new("testuser").unwrap(),
            password_hash,
            chrono::Utc::now(),
        );
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "email": "test@example.com",
                "password": "wrongpassword"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
