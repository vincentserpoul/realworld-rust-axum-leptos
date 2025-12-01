//! Get current user use case

use crate::{
    DomainError, DomainResult, User, UserId, UserView,
    repositories::UsersRepository,
};

/// Get the current authenticated user
///
/// # Business Rules
/// - User must exist
pub async fn get_current_user<U>(
    users_repo: &U,
    user_id: UserId,
) -> DomainResult<User>
where
    U: UsersRepository,
{
    users_repo
        .get_user_by_id(user_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "user" })?
        .ok_or(DomainError::NotFound { entity: "user" })
}

/// Get the current authenticated user as a view
///
/// # Business Rules
/// - User must exist
/// - Token is added by the caller
pub async fn get_current_user_view<U>(
    users_repo: &U,
    user_id: UserId,
) -> DomainResult<UserView>
where
    U: UsersRepository,
{
    let user = get_current_user(users_repo, user_id).await?;
    Ok(user.to_view(None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUsersRepository;
    use crate::{Email, PasswordHash, Username};
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
    async fn test_get_current_user_success() {
        let (users_repo, user) = setup().await;

        let result = get_current_user(&users_repo, user.id).await;

        assert!(result.is_ok());
        let found = result.unwrap();
        assert_eq!(found.id, user.id);
    }

    #[tokio::test]
    async fn test_get_current_user_not_found() {
        let (users_repo, _) = setup().await;
        let unknown_id = UserId::random();

        let result = get_current_user(&users_repo, unknown_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "user" })));
    }

    #[tokio::test]
    async fn test_get_current_user_view_success() {
        let (users_repo, user) = setup().await;

        let result = get_current_user_view(&users_repo, user.id).await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.email.as_str(), user.email.as_str());
        assert!(view.token.is_none()); // Token added by API layer
    }
}
