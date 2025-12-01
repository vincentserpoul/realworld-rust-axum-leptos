//! Get profile use case

use crate::{
    DomainError, DomainResult, Profile, UserId,
    repositories::UsersRepository,
};

/// Get a user's profile by username
///
/// # Business Rules
/// - User must exist
/// - Following status is relative to the viewer (false if no viewer)
pub async fn get_profile<U>(
    users_repo: &U,
    username: &str,
    viewer_id: Option<UserId>,
) -> DomainResult<Profile>
where
    U: UsersRepository,
{
    let user = users_repo
        .get_user_by_username(username)
        .await
        .map_err(|_| DomainError::NotFound { entity: "profile" })?
        .ok_or(DomainError::NotFound { entity: "profile" })?;

    let following = match viewer_id {
        Some(viewer) => users_repo
            .is_following(viewer, user.id)
            .await
            .unwrap_or(false),
        None => false,
    };

    Ok(user.to_profile(following))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUsersRepository;
    use crate::{Email, PasswordHash, User, Username};
    use chrono::Utc;

    async fn setup() -> (InMemoryUsersRepository, User) {
        let users_repo = InMemoryUsersRepository::new();
        let user = User::new(
            UserId::random(),
            Email::parse("test@example.com").unwrap(),
            Username::new("testuser").unwrap(),
            PasswordHash::new("hash").unwrap(),
            Utc::now(),
        );
        let user = users_repo.create_user(user).await.unwrap();
        (users_repo, user)
    }

    #[tokio::test]
    async fn test_get_profile_success() {
        let (users_repo, user) = setup().await;

        let result = get_profile(&users_repo, user.username.as_str(), None).await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.username.as_str(), "testuser");
        assert!(!profile.following);
    }

    #[tokio::test]
    async fn test_get_profile_not_found() {
        let (users_repo, _) = setup().await;

        let result = get_profile(&users_repo, "nonexistent", None).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "profile" })));
    }

    #[tokio::test]
    async fn test_get_profile_with_following() {
        let (users_repo, user) = setup().await;
        let viewer_id = UserId::random();

        users_repo.follow_user(viewer_id, user.id).await.unwrap();

        let result = get_profile(&users_repo, user.username.as_str(), Some(viewer_id)).await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert!(profile.following);
    }

    #[tokio::test]
    async fn test_get_profile_not_following() {
        let (users_repo, user) = setup().await;
        let viewer_id = UserId::random();

        let result = get_profile(&users_repo, user.username.as_str(), Some(viewer_id)).await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert!(!profile.following);
    }
}
