use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
    routing::post,
};
use domain::{
    Profile, ProfileEnvelope, User, UserId,
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
    Router::<AppState<U, A, C>>::new().route("/:username", get(get_profile)).route(
        "/:username/follow",
        post(follow_profile).delete(unfollow_profile),
    )
}

async fn get_profile<U, A, C>(
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
    let user = find_user_by_username(&state, &username).await?;
    let profile = to_profile(&state, user, viewer_id).await?;
    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn follow_profile<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(username): Path<String>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ProfileEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let target = find_user_by_username(&state, &username).await?;
    Profile::validate_follow_action(&target.id, &user.id)
        .map_err(|_| ApiError::validation("cannot follow yourself"))?;

    state.use_cases.users_repo.follow_user(user.id, target.id).await?;

    let profile = to_profile(&state, target, Some(user.id)).await?;
    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn unfollow_profile<U, A, C>(
    State(state): State<AppState<U, A, C>>,
    Path(username): Path<String>,
    CurrentUser { user, .. }: CurrentUser,
) -> ApiResult<Json<ProfileEnvelope>>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let target = find_user_by_username(&state, &username).await?;
    Profile::validate_follow_action(&target.id, &user.id)
        .map_err(|_| ApiError::validation("cannot unfollow yourself"))?;

    state.use_cases.users_repo.unfollow_user(user.id, target.id).await?;

    let profile = to_profile(&state, target, Some(user.id)).await?;
    Ok(Json(ProfileEnvelope::from(profile)))
}

async fn find_user_by_username<U, A, C>(state: &AppState<U, A, C>, username: &str) -> ApiResult<User>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    state.use_cases.users_repo
        .get_user_by_username(username)
        .await?
        .ok_or_else(|| ApiError::not_found("profile"))
}

async fn to_profile<U, A, C>(state: &AppState<U, A, C>, user: User, viewer_id: Option<UserId>) -> ApiResult<Profile>
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    let following = match viewer_id {
        Some(viewer) => is_following_user(state, user.id, viewer).await,
        None => false,
    };
    Ok(user.to_profile(following))
}

async fn is_following_user<U, A, C>(state: &AppState<U, A, C>, target_id: UserId, follower_id: UserId) -> bool
where
    U: domain::repositories::UsersRepository + Clone,
    A: domain::repositories::ArticlesRepository + Clone,
    C: domain::repositories::CommentsRepository + Clone,
{
    state.use_cases.users_repo
        .is_following(follower_id, target_id)
        .await
        .unwrap_or(false)
}


#[cfg(test)]
mod tests {
    use super::*;
    use domain::repositories::UsersRepository;
    use std::collections::{HashMap, HashSet};
    use domain::{add_follower, remove_follower, Email, PasswordHash, Profile, Username};

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
    async fn test_find_user_by_username() {
        let state = AppState::default();
        let user_id = UserId::random();
        let user = create_test_user(user_id, "testuser", "test@example.com");
        state.use_cases.users_repo.create_user(user).await.unwrap();
        
        let result = find_user_by_username(&state, "testuser").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_user_by_username_not_found() {
        let state = AppState::default();
        
        let result = find_user_by_username(&state, "nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_following_user_true() {
        let state = AppState::default();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        state.use_cases.users_repo.follow_user(follower_id, target_id).await.unwrap();
        
        let following = is_following_user(&state, target_id, follower_id).await;
        assert!(following);
    }

    #[tokio::test]
    async fn test_is_following_user_false() {
        let state = AppState::default();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        let following = is_following_user(&state, target_id, follower_id).await;
        assert!(!following);
    }

    #[tokio::test]
    async fn test_to_profile_with_following() {
        let state = AppState::default();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        let user = create_test_user(target_id, "target", "target@example.com");
        
        state.use_cases.users_repo.follow_user(follower_id, target_id).await.unwrap();
        
        let profile = to_profile(&state, user, Some(follower_id)).await;
        assert!(profile.is_ok());
    }

    #[tokio::test]
    async fn test_to_profile_without_viewer() {
        let state = AppState::default();
        let target_id = UserId::random();
        let user = create_test_user(target_id, "target", "target@example.com");
        
        let profile = to_profile(&state, user, None).await;
        assert!(profile.is_ok());
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

    #[test]
    fn test_add_follower() {
        let mut followers: HashMap<UserId, HashSet<UserId>> = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        add_follower(&mut followers, target_id, follower_id);
        
        assert!(followers.contains_key(&target_id));
        assert!(followers.get(&target_id).unwrap().contains(&follower_id));
    }

    #[test]
    fn test_add_follower_to_existing() {
        let mut followers: HashMap<UserId, HashSet<UserId>> = HashMap::new();
        let target_id = UserId::random();
        let follower1_id = UserId::random();
        let follower2_id = UserId::random();
        
        add_follower(&mut followers, target_id, follower1_id);
        add_follower(&mut followers, target_id, follower2_id);
        
        let set = followers.get(&target_id).unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains(&follower1_id));
        assert!(set.contains(&follower2_id));
    }

    #[test]
    fn test_remove_follower() {
        let mut followers: HashMap<UserId, HashSet<UserId>> = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        let mut set = HashSet::new();
        set.insert(follower_id);
        followers.insert(target_id, set);
        
        remove_follower(&mut followers, target_id, follower_id);
        
        assert!(followers.get(&target_id).unwrap().is_empty());
    }

    #[test]
    fn test_remove_follower_not_following() {
        let mut followers: HashMap<UserId, HashSet<UserId>> = HashMap::new();
        let target_id = UserId::random();
        let follower_id = UserId::random();
        
        remove_follower(&mut followers, target_id, follower_id);
        // Should not panic
    }
}
