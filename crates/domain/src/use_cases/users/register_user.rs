//! Register user use case

use chrono::{DateTime, Utc};

use crate::{
    DomainError, DomainResult, Email, PasswordHash, User, UserId, UserView, Username,
    repositories::UsersRepository,
};

/// Input for registering a new user
#[derive(Debug, Clone)]
pub struct RegisterUserInput {
    pub username: String,
    pub email: String,
    pub password_hash: PasswordHash,
}

/// Output from registering a new user
#[derive(Debug, Clone)]
pub struct RegisterUserOutput {
    pub user: User,
    pub view: UserView,
}

/// Register a new user
///
/// # Business Rules
/// - Email must be valid and unique
/// - Username must be non-empty and unique
/// - Password is already hashed by the caller (infrastructure concern)
pub async fn register_user<U>(
    users_repo: &U,
    input: RegisterUserInput,
    now: DateTime<Utc>,
) -> DomainResult<RegisterUserOutput>
where
    U: UsersRepository,
{
    let username = Username::new(input.username)?;
    let email = Email::parse(input.email)?;

    // Check for existing user with same email
    if users_repo
        .get_user_by_email(email.as_str())
        .await
        .map_err(|_| DomainError::Conflict { entity: "email" })?
        .is_some()
    {
        return Err(DomainError::Conflict { entity: "email" });
    }

    // Check for existing user with same username
    if users_repo
        .get_user_by_username(username.as_str())
        .await
        .map_err(|_| DomainError::Conflict { entity: "username" })?
        .is_some()
    {
        return Err(DomainError::Conflict { entity: "username" });
    }

    let user = User::new(UserId::random(), email, username, input.password_hash, now);

    let created = users_repo
        .create_user(user)
        .await
        .map_err(|_| DomainError::Conflict { entity: "user" })?;

    // Token will be added by the API layer
    let view = created.to_view(None);

    Ok(RegisterUserOutput {
        user: created,
        view,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUsersRepository;

    fn password_hash() -> PasswordHash {
        PasswordHash::new("hashed:password123").unwrap()
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let users_repo = InMemoryUsersRepository::new();
        let input = RegisterUserInput {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_hash(),
        };

        let result = register_user(&users_repo, input, Utc::now()).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.user.username.as_str(), "testuser");
        assert_eq!(output.user.email.as_str(), "test@example.com");
    }

    #[tokio::test]
    async fn test_register_user_duplicate_email() {
        let users_repo = InMemoryUsersRepository::new();

        // Create first user
        let user = User::new(
            UserId::random(),
            Email::parse("test@example.com").unwrap(),
            Username::new("existing").unwrap(),
            password_hash(),
            Utc::now(),
        );
        users_repo.create_user(user).await.unwrap();

        // Try to register with same email
        let input = RegisterUserInput {
            username: "newuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_hash(),
        };

        let result = register_user(&users_repo, input, Utc::now()).await;

        assert!(matches!(result, Err(DomainError::Conflict { entity: "email" })));
    }

    #[tokio::test]
    async fn test_register_user_duplicate_username() {
        let users_repo = InMemoryUsersRepository::new();

        // Create first user
        let user = User::new(
            UserId::random(),
            Email::parse("existing@example.com").unwrap(),
            Username::new("testuser").unwrap(),
            password_hash(),
            Utc::now(),
        );
        users_repo.create_user(user).await.unwrap();

        // Try to register with same username
        let input = RegisterUserInput {
            username: "testuser".to_string(),
            email: "new@example.com".to_string(),
            password_hash: password_hash(),
        };

        let result = register_user(&users_repo, input, Utc::now()).await;

        assert!(matches!(result, Err(DomainError::Conflict { entity: "username" })));
    }

    #[tokio::test]
    async fn test_register_user_invalid_email() {
        let users_repo = InMemoryUsersRepository::new();
        let input = RegisterUserInput {
            username: "testuser".to_string(),
            email: "invalid-email".to_string(),
            password_hash: password_hash(),
        };

        let result = register_user(&users_repo, input, Utc::now()).await;

        assert!(matches!(result, Err(DomainError::InvalidEmail)));
    }
}
