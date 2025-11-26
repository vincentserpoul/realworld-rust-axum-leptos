use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::Utc;
use domain::{
    AuthToken, Email, PasswordHash, PlainPassword, User, UserEnvelope, UserId, Username,
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
        .route("/", post(register_user))
        .route("/login", post(login_user))
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

async fn register_user<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<impl IntoResponse>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let RegisterPayload {
        username,
        email,
        password,
    } = req.user;

    let username = Username::new(username)?;
    let email = Email::parse(email)?;
    let password = PlainPassword::new(password)?;

    let password_hash = hash_password(&password)?;
    let now = Utc::now();
    let user = User::new(UserId::random(), email.clone(), username, password_hash, now);

    if state.use_cases.users_repo.get_user_by_email(email.as_str()).await?.is_some() {
        return Err(ApiError::conflict("email already registered"));
    }
    
    let user = state.use_cases.users_repo.create_user(user).await?;

    let token = issue_token()?;
    remember_session(&state, &token, user.id).await;
    let view = user.to_view(Some(token));

    Ok((StatusCode::CREATED, Json(UserEnvelope::from(view))))
}

async fn login_user<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<impl IntoResponse>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let LoginPayload { email, password } = req.user;
    let email = Email::parse(email)?;
    let password = PlainPassword::new(password)?;
    let password_hash = hash_password(&password)?;

    let Some(user) = state.use_cases.users_repo.get_user_by_email(email.as_str()).await? else {
        return Err(ApiError::unauthorized("invalid credentials"));
    };

    if user.password_hash != password_hash {
        return Err(ApiError::unauthorized("invalid credentials"));
    }

    let token = issue_token()?;
    remember_session(&state, &token, user.id).await;
    let view = user.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

pub(crate) fn hash_password(password: &PlainPassword) -> ApiResult<PasswordHash> {
    let salted = format!("hashed:{}", password.as_str());
    PasswordHash::new(salted).map_err(ApiError::from)
}

fn issue_token() -> ApiResult<AuthToken> {
    AuthToken::new(Uuid::new_v4().to_string()).map_err(ApiError::from)
}

async fn remember_session<U, A, C>(state: &AppState<U, A, C>, token: &AuthToken, user_id: UserId)
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
    use domain::repositories::UsersRepository;
    use axum::{body::Body, http::{Request, StatusCode}};
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
