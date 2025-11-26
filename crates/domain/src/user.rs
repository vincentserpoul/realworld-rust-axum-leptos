use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, DomainResult};
use crate::identifiers::UserId;
use crate::profile::Profile;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() || !trimmed.contains('@') {
            return Err(DomainError::InvalidEmail);
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let trimmed = value.into().trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidUsername);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageUrl(String);

impl ImageUrl {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let trimmed = value.into().trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidImageUrl);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        if value.trim().len() < 8 {
            return Err(DomainError::InvalidPassword);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let trimmed = value.into().trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidPassword);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for PasswordHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        PasswordHash::new(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let trimmed = value.into().trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::UnauthorizedAction);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub bio: Option<String>,
    pub image: Option<ImageUrl>,
    #[serde(skip, default)]
    pub password_hash: PasswordHash,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        id: UserId,
        email: Email,
        username: Username,
        password_hash: PasswordHash,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            username,
            bio: None,
            image: None,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn apply_update(&mut self, changes: UpdateUserInput, now: DateTime<Utc>) {
        if let Some(email) = changes.email {
            self.email = email;
        }
        if let Some(username) = changes.username {
            self.username = username;
        }
        if let Some(bio) = changes.bio {
            self.bio = bio;
        }
        if let Some(image) = changes.image {
            self.image = image;
        }
        if let Some(password_hash) = changes.password_hash {
            self.password_hash = password_hash;
        }
        self.updated_at = now;
    }

    pub fn to_profile(&self, following: bool) -> Profile {
        Profile::new(
            self.username.clone(),
            self.bio.clone(),
            self.image.clone(),
            following,
        )
    }

    pub fn to_view(&self, token: Option<AuthToken>) -> UserView {
        UserView {
            email: self.email.clone(),
            token,
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserInput {
    pub email: Email,
    pub username: Username,
    pub password: PlainPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUserInput {
    pub email: Email,
    pub password: PlainPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateUserInput {
    pub email: Option<Email>,
    pub username: Option<Username>,
    pub bio: Option<Option<String>>,
    pub image: Option<Option<ImageUrl>>,
    pub password: Option<PlainPassword>,
    pub password_hash: Option<PasswordHash>,
}

impl UpdateUserInput {
    pub fn with_password_hash(mut self, password_hash: PasswordHash) -> Self {
        self.password_hash = Some(password_hash);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserView {
    pub email: Email,
    pub token: Option<AuthToken>,
    pub username: Username,
    pub bio: Option<String>,
    pub image: Option<ImageUrl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEnvelope {
    pub user: UserView,
}

impl From<UserView> for UserEnvelope {
    fn from(value: UserView) -> Self {
        Self { user: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn user_id() -> UserId {
        UserId::random()
    }

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn password() -> PasswordHash {
        PasswordHash::new("hashed-password").unwrap()
    }

    #[test]
    fn email_parse_validates_format() {
        assert!(Email::parse("test@example.com").is_ok());
        assert_eq!(
            Email::parse("invalid").unwrap_err(),
            DomainError::InvalidEmail
        );
    }

    #[test]
    fn plain_password_requires_min_length() {
        assert!(PlainPassword::new("12345678").is_ok());
        assert_eq!(
            PlainPassword::new("short").unwrap_err(),
            DomainError::InvalidPassword
        );
    }

    #[test]
    fn user_apply_update_overrides_fields() {
        let mut user = User::new(
            user_id(),
            Email::parse("old@example.com").unwrap(),
            Username::new("old-name").unwrap(),
            password(),
            now(),
        );
        let new_time = user.created_at + Duration::minutes(5);
        let changes = UpdateUserInput {
            email: Some(Email::parse("new@example.com").unwrap()),
            username: Some(Username::new("new-name").unwrap()),
            bio: Some(Some("bio".into())),
            image: Some(Some(ImageUrl::new("https://example.com").unwrap())),
            password: None,
            password_hash: Some(PasswordHash::new("new-hash").unwrap()),
        };

        user.apply_update(changes, new_time);

        assert_eq!(user.email.as_str(), "new@example.com");
        assert_eq!(user.username.as_str(), "new-name");
        assert_eq!(user.bio.as_deref(), Some("bio"));
        assert_eq!(
            user.image.as_ref().map(ImageUrl::as_str),
            Some("https://example.com")
        );
        assert_eq!(user.password_hash.as_str(), "new-hash");
        assert_eq!(user.updated_at, new_time);
    }

    #[test]
    fn user_to_profile_sets_following_flag() {
        let user = User::new(
            user_id(),
            Email::parse("user@example.com").unwrap(),
            Username::new("jake").unwrap(),
            password(),
            now(),
        );
        let profile = user.to_profile(true);
        assert!(profile.following);
        assert_eq!(profile.username.as_str(), "jake");
    }

    #[test]
    fn user_to_view_clones_public_fields() {
        let mut user = User::new(
            user_id(),
            Email::parse("user@example.com").unwrap(),
            Username::new("jake").unwrap(),
            password(),
            now(),
        );
        user.bio = Some("hi".into());
        let token = AuthToken::new("jwt").unwrap();
        let view = user.to_view(Some(token.clone()));
        assert_eq!(view.email.as_str(), "user@example.com");
        assert_eq!(view.username.as_str(), "jake");
        assert_eq!(view.bio.as_deref(), Some("hi"));
        assert_eq!(view.token.unwrap().as_str(), token.as_str());
    }
}
