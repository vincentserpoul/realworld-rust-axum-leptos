use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use domain::{
    Email, ImageUrl, PlainPassword, UpdateUserInput, UserEnvelope, Username,
};
use serde::Deserialize;

use crate::{
    auth::CurrentUser,
    error::{ApiError, ApiResult},
    state::AppState,
};

use super::users::hash_password;

pub fn router<U, A, C>() -> Router<AppState<U, A, C>>
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new().route("/user", get(get_current_user).put(update_current_user))
}

async fn get_current_user(
    CurrentUser { user, token }: CurrentUser,
) -> ApiResult<Json<UserEnvelope>> {
    let view = user.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

#[derive(Debug, Deserialize, Default)]
struct UpdateUserRequest {
    user: UpdateUserPayload,
}

#[derive(Debug, Deserialize, Default)]
struct UpdateUserPayload {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,
}

async fn update_current_user<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    CurrentUser { user, token }: CurrentUser,
    Json(req): Json<UpdateUserRequest>,
) -> ApiResult<Json<UserEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let UpdateUserPayload {
        email,
        username,
        password,
        bio,
        image,
    } = req.user;

    let current_id = user.id;

    let parsed_email = if let Some(email) = email {
        let email = Email::parse(email)?;
        if let Some(existing_user) = state.use_cases.users_repo.get_user_by_email(email.as_str()).await?
            && existing_user.id != current_id
        {
            return Err(ApiError::conflict("email already registered"));
        }
        Some(email)
    } else {
        None
    };

    let mut existing = user;

    let mut changes = UpdateUserInput::default();

    if let Some(email) = parsed_email {
        changes.email = Some(email);
    }

    if let Some(username) = username {
        changes.username = Some(Username::new(username)?);
    }

    if let Some(bio) = bio {
        changes.bio = Some(Some(bio));
    }

    if let Some(image) = image {
        changes.image = Some(Some(ImageUrl::new(image)?));
    }

    if let Some(password) = password {
        let password = PlainPassword::new(password)?;
        let password_hash = hash_password(&password)?;
        changes.password_hash = Some(password_hash);
    }

    existing.apply_update(changes, Utc::now());
    let updated = state.use_cases.users_repo.update_user(existing).await?;
    let view = updated.to_view(Some(token));
    Ok(Json(UserEnvelope::from(view)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::repositories::UsersRepository;
    use axum::{body::Body, http::{Request, StatusCode}};
    use domain::{AuthToken, Email, PasswordHash, User, UserId, Username};
    use tower::ServiceExt;

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
    async fn test_get_current_user() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let app = router().with_state(state);
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_current_user_without_auth() {
        let state = AppState::default();
        let app = router().with_state(state);
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert!(!response.status().is_success());
    }

    #[tokio::test]
    async fn test_update_current_user_email() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "testuser", "old@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "email": "new@example.com"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_current_user_username() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "oldname", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "username": "newname"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_current_user_password() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "password": "newpassword123"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_current_user_bio_and_image() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "bio": "My bio",
                "image": "https://example.com/avatar.jpg"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_current_user_duplicate_email_fails() {
        let state = AppState::default();
        let user1_id = UserId::random();
        let user2_id = UserId::random();
        
        let user1 = create_test_user(user1_id, "user1", "user1@example.com");
        let user2 = create_test_user(user2_id, "user2", "user2@example.com");
        
        state.use_cases.users_repo.create_user(user1).await.unwrap();
        state.use_cases.users_repo.create_user(user2).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user1_id);
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "email": "user2@example.com"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_update_current_user_all_fields() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "oldname", "old@example.com");
        
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let token = AuthToken::new("test-token".to_string()).unwrap();
        state.sessions.write().await.insert(token.as_str().to_owned(), user_id);
        
        let app = router().with_state(state);
        
        let payload = serde_json::json!({
            "user": {
                "email": "new@example.com",
                "username": "newname",
                "password": "newpassword123",
                "bio": "My new bio",
                "image": "https://example.com/new-avatar.jpg"
            }
        });
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/user")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}
