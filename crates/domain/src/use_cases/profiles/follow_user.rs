//! Follow user use case

use crate::{
    DomainError, DomainResult, Profile, UserId,
    repositories::UsersRepository,
};

/// Follow a user
///
/// # Business Rules
/// - Target user must exist
/// - Cannot follow yourself
/// - Following already-followed user is idempotent
pub async fn follow_user<U>(
    users_repo: &U,
    username: &str,
    follower_id: UserId,
) -> DomainResult<Profile>
where
    U: UsersRepository,
{
    let target = users_repo
        .get_user_by_username(username)
        .await
        .map_err(|_| DomainError::NotFound { entity: "profile" })?
        .ok_or(DomainError::NotFound { entity: "profile" })?;

    // Cannot follow yourself
    Profile::validate_follow_action(&target.id, &follower_id)?;

    users_repo
        .follow_user(follower_id, target.id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "profile" })?;

    // Return profile with following = true
    Ok(target.to_profile(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUsersRepository;
    use crate::{Email, PasswordHash, User, Username};
    use chrono::Utc;

    async fn setup() -> (InMemoryUsersRepository, User, UserId) {
        let users_repo = InMemoryUsersRepository::new();
        let target = User::new(
            UserId::random(),
            Email::parse("target@example.com").unwrap(),
            Username::new("target").unwrap(),
            PasswordHash::new("hash").unwrap(),
            Utc::now(),
        );
        let target = users_repo.create_user(target).await.unwrap();
        let follower_id = UserId::random();
        (users_repo, target, follower_id)
    }

    #[tokio::test]
    async fn test_follow_user_success() {
        let (users_repo, target, follower_id) = setup().await;

        let result = follow_user(&users_repo, target.username.as_str(), follower_id).await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert!(profile.following);
        assert_eq!(profile.username.as_str(), "target");
    }

    #[tokio::test]
    async fn test_follow_user_not_found() {
        let (users_repo, _, follower_id) = setup().await;

        let result = follow_user(&users_repo, "nonexistent", follower_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "profile" })));
    }

    #[tokio::test]
    async fn test_follow_user_self() {
        let (users_repo, target, _) = setup().await;

        let result = follow_user(&users_repo, target.username.as_str(), target.id).await;

        assert!(matches!(result, Err(DomainError::UnauthorizedAction)));
    }

    #[tokio::test]
    async fn test_follow_user_idempotent() {
        let (users_repo, target, follower_id) = setup().await;

        // Follow twice
        follow_user(&users_repo, target.username.as_str(), follower_id).await.unwrap();
        let result = follow_user(&users_repo, target.username.as_str(), follower_id).await;

        assert!(result.is_ok());
        let profile = result.unwrap();
        assert!(profile.following);
    }
}
