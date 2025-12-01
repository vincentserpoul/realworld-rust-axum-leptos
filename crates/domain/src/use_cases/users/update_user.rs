//! Update user use case

use chrono::{DateTime, Utc};

use crate::{
    DomainError, DomainResult, Email, PasswordHash, UserId, UserView, Username,
    repositories::UsersRepository,
    user::{ImageUrl, UpdateUserInput as DomainUpdateUserInput},
};

/// Input for updating a user
#[derive(Debug, Clone, Default)]
pub struct UpdateUserInput {
    pub email: Option<String>,
    pub username: Option<String>,
    pub bio: Option<Option<String>>,
    pub image: Option<Option<String>>,
    pub password_hash: Option<PasswordHash>,
}

/// Update the current user
///
/// # Business Rules
/// - User must exist
/// - Email and username must remain unique if changed
/// - All fields are optional
pub async fn update_user<U>(
    users_repo: &U,
    user_id: UserId,
    input: UpdateUserInput,
    now: DateTime<Utc>,
) -> DomainResult<UserView>
where
    U: UsersRepository,
{
    let mut user = users_repo
        .get_user_by_id(user_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "user" })?
        .ok_or(DomainError::NotFound { entity: "user" })?;

    // Validate and parse email if provided
    let email = match input.email {
        Some(email_str) => {
            let parsed = Email::parse(email_str)?;
            // Check uniqueness if email changed
            if parsed.as_str() != user.email.as_str()
                && users_repo
                    .get_user_by_email(parsed.as_str())
                    .await
                    .map_err(|_| DomainError::Conflict { entity: "email" })?
                    .is_some()
            {
                return Err(DomainError::Conflict { entity: "email" });
            }
            Some(parsed)
        }
        None => None,
    };

    // Validate and parse username if provided
    let username = match input.username {
        Some(username_str) => {
            let parsed = Username::new(username_str)?;
            // Check uniqueness if username changed
            if parsed.as_str() != user.username.as_str()
                && users_repo
                    .get_user_by_username(parsed.as_str())
                    .await
                    .map_err(|_| DomainError::Conflict { entity: "username" })?
                    .is_some()
            {
                return Err(DomainError::Conflict { entity: "username" });
            }
            Some(parsed)
        }
        None => None,
    };

    // Parse image URL if provided
    let image = match input.image {
        Some(Some(url)) => Some(Some(ImageUrl::new(url)?)),
        Some(None) => Some(None),
        None => None,
    };

    let domain_input = DomainUpdateUserInput {
        email,
        username,
        bio: input.bio,
        image,
        password: None,
        password_hash: input.password_hash,
    };

    user.apply_update(domain_input, now);

    let updated = users_repo
        .update_user(user)
        .await
        .map_err(|_| DomainError::NotFound { entity: "user" })?;

    // Token will be added by the API layer
    Ok(updated.to_view(None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUsersRepository;
    use crate::User;
    use chrono::Utc;

    fn password_hash() -> PasswordHash {
        PasswordHash::new("hash").unwrap()
    }

    async fn setup() -> (InMemoryUsersRepository, User) {
        let users_repo = InMemoryUsersRepository::new();
        let user = User::new(
            UserId::random(),
            Email::parse("test@example.com").unwrap(),
            Username::new("testuser").unwrap(),
            password_hash(),
            Utc::now(),
        );
        let user = users_repo.create_user(user).await.unwrap();
        (users_repo, user)
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let (users_repo, user) = setup().await;
        let input = UpdateUserInput {
            bio: Some(Some("New bio".to_string())),
            ..Default::default()
        };

        let result = update_user(&users_repo, user.id, input, Utc::now()).await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.bio.as_deref(), Some("New bio"));
    }

    #[tokio::test]
    async fn test_update_user_email_conflict() {
        let (users_repo, user) = setup().await;

        // Create another user with different email
        let other_user = User::new(
            UserId::random(),
            Email::parse("other@example.com").unwrap(),
            Username::new("other").unwrap(),
            password_hash(),
            Utc::now(),
        );
        users_repo.create_user(other_user).await.unwrap();

        // Try to update to existing email
        let input = UpdateUserInput {
            email: Some("other@example.com".to_string()),
            ..Default::default()
        };

        let result = update_user(&users_repo, user.id, input, Utc::now()).await;

        assert!(matches!(result, Err(DomainError::Conflict { entity: "email" })));
    }

    #[tokio::test]
    async fn test_update_user_same_email_ok() {
        let (users_repo, user) = setup().await;
        let input = UpdateUserInput {
            email: Some("test@example.com".to_string()), // Same email
            ..Default::default()
        };

        let result = update_user(&users_repo, user.id, input, Utc::now()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let (users_repo, _) = setup().await;
        let unknown_id = UserId::random();
        let input = UpdateUserInput::default();

        let result = update_user(&users_repo, unknown_id, input, Utc::now()).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "user" })));
    }
}
