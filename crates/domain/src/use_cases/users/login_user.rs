//! Login user use case

use crate::{
    DomainError, DomainResult, Email, PasswordHash, User, UserView,
    repositories::UsersRepository,
};

/// Input for logging in a user
#[derive(Debug, Clone)]
pub struct LoginUserInput {
    pub email: String,
    pub password_hash: PasswordHash,
}

/// Output from logging in a user
#[derive(Debug, Clone)]
pub struct LoginUserOutput {
    pub user: User,
    pub view: UserView,
}

/// Login a user with email and password
///
/// # Business Rules
/// - Email must exist in the system
/// - Password hash must match stored hash
pub async fn login_user<U>(
    users_repo: &U,
    input: LoginUserInput,
) -> DomainResult<LoginUserOutput>
where
    U: UsersRepository,
{
    let email = Email::parse(input.email)?;

    let user = users_repo
        .get_user_by_email(email.as_str())
        .await
        .map_err(|_| DomainError::UnauthorizedAction)?
        .ok_or(DomainError::UnauthorizedAction)?;

    if user.password_hash != input.password_hash {
        return Err(DomainError::UnauthorizedAction);
    }

    // Token will be added by the API layer
    let view = user.to_view(None);

    Ok(LoginUserOutput { user, view })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryUsersRepository;
    use crate::{UserId, Username};
    use chrono::Utc;

    fn password_hash() -> PasswordHash {
        PasswordHash::new("hashed:password123").unwrap()
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
    async fn test_login_user_success() {
        let (users_repo, user) = setup().await;
        let input = LoginUserInput {
            email: "test@example.com".to_string(),
            password_hash: password_hash(),
        };

        let result = login_user(&users_repo, input).await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.user.id, user.id);
    }

    #[tokio::test]
    async fn test_login_user_wrong_email() {
        let (users_repo, _) = setup().await;
        let input = LoginUserInput {
            email: "wrong@example.com".to_string(),
            password_hash: password_hash(),
        };

        let result = login_user(&users_repo, input).await;

        assert!(matches!(result, Err(DomainError::UnauthorizedAction)));
    }

    #[tokio::test]
    async fn test_login_user_wrong_password() {
        let (users_repo, _) = setup().await;
        let input = LoginUserInput {
            email: "test@example.com".to_string(),
            password_hash: PasswordHash::new("wrong-hash").unwrap(),
        };

        let result = login_user(&users_repo, input).await;

        assert!(matches!(result, Err(DomainError::UnauthorizedAction)));
    }

    #[tokio::test]
    async fn test_login_user_invalid_email() {
        let (users_repo, _) = setup().await;
        let input = LoginUserInput {
            email: "invalid".to_string(),
            password_hash: password_hash(),
        };

        let result = login_user(&users_repo, input).await;

        assert!(matches!(result, Err(DomainError::InvalidEmail)));
    }
}
