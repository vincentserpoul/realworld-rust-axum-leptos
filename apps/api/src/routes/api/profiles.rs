use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
    routing::post,
};
use domain::{
    ProfileEnvelope,
    use_cases::{follow_user, get_profile, unfollow_user},
};

use crate::{
    auth::CurrentUser,
    error::{ApiError, ApiResult},
    state::AppState,
};

pub fn router<U, A, C>() -> Router<AppState<U, A, C>>
where
    U: domain::repositories::UsersRepository + Clone + 'static,
    A: domain::repositories::ArticlesRepository + Clone + 'static,
    C: domain::repositories::CommentsRepository + Clone + 'static,
{
    Router::<AppState<U, A, C>>::new().route("/{username}", get(get_profile_handler)).route(
        "/{username}/follow",
        post(follow_profile_handler).delete(unfollow_profile_handler),
    )
}

async fn get_profile_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(username): Path<String>,
    current_user: Option<CurrentUser>,
) -> ApiResult<Json<ProfileEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let viewer_id = current_user.as_ref().map(|current| current.user.id);

    let profile = get_profile(&state.use_cases.users_repo, &username, viewer_id)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn follow_profile_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(username): Path<String>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ProfileEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let profile = follow_user(&state.use_cases.users_repo, &username, user.id)
        .await
        .map_err(|e| match e {
            domain::DomainError::NotFound { .. } => ApiError::not_found("profile"),
            domain::DomainError::UnauthorizedAction => ApiError::validation("cannot follow yourself"),
            _ => ApiError::from(e),
        })?;

    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn unfollow_profile_handler<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(username): Path<String>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ProfileEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let profile = unfollow_user(&state.use_cases.users_repo, &username, user.id)
        .await
        .map_err(|e| match e {
            domain::DomainError::NotFound { .. } => ApiError::not_found("profile"),
            domain::DomainError::UnauthorizedAction => ApiError::validation("cannot unfollow yourself"),
            _ => ApiError::from(e),
        })?;

    Ok(Json(ProfileEnvelope::from(profile)))
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use domain::repositories::UsersRepository;
    use domain::{AuthToken, Email, PasswordHash, Profile, User, UserId, Username};
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
    async fn test_get_profile_success() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        state.use_cases.users_repo.create_user(user).await.unwrap();

        let app = router().with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/testuser")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_profile_not_found() {
        let state = AppState::default();
        let app = router().with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_follow_user_success() {
        let state = AppState::default();
        let follower_id = UserId::random();
        let target_id = UserId::random();

        let follower = create_test_user(follower_id, "follower", "follower@example.com");
        let target = create_test_user(target_id, "target", "target@example.com");

        state.use_cases.users_repo.create_user(follower).await.unwrap();
        state.use_cases.users_repo.create_user(target).await.unwrap();

        let token = AuthToken::new("test-token".to_string()).unwrap();
        state
            .sessions
            .write()
            .await
            .insert(token.as_str().to_owned(), follower_id);

        let app = router().with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/target/follow")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_unfollow_user_success() {
        let state = AppState::default();
        let follower_id = UserId::random();
        let target_id = UserId::random();

        let follower = create_test_user(follower_id, "follower", "follower@example.com");
        let target = create_test_user(target_id, "target", "target@example.com");

        state.use_cases.users_repo.create_user(follower).await.unwrap();
        state.use_cases.users_repo.create_user(target).await.unwrap();
        state
            .use_cases
            .users_repo
            .follow_user(follower_id, target_id)
            .await
            .unwrap();

        let token = AuthToken::new("test-token".to_string()).unwrap();
        state
            .sessions
            .write()
            .await
            .insert(token.as_str().to_owned(), follower_id);

        let app = router().with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/target/follow")
                    .header("authorization", format!("Token {}", token.as_str()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_validate_follow_action_success() {
        let target_id = UserId::random();
        let follower_id = UserId::random();

        let result = Profile::validate_follow_action(&target_id, &follower_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_follow_action_self_follow() {
        let user_id = UserId::random();

        let result = Profile::validate_follow_action(&user_id, &user_id);
        assert!(result.is_err());
    }
}
