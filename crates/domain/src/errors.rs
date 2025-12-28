use thiserror::Error;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("invalid email address")]
    InvalidEmail,
    #[error("username must not be empty")]
    InvalidUsername,
    #[error("password must contain at least 8 characters")]
    InvalidPassword,
    #[error("image url must not be empty")]
    InvalidImageUrl,
    #[error("title must not be empty")]
    InvalidTitle,
    #[error("description must not be empty")]
    InvalidDescription,
    #[error("body must not be empty")]
    InvalidBody,
    #[error("slug must not be empty")]
    InvalidSlug,
    #[error("tag must not be empty")]
    InvalidTag,
    #[error("limit must be between 1 and 50")]
    LimitOutOfRange,
    #[error("offset cannot be negative")]
    NegativeOffset,
    #[error("requested entity `{entity}` was not found")]
    NotFound { entity: &'static str },
    #[error("`{entity}` already exists")]
    Conflict { entity: &'static str },
    #[error("operation is not allowed for the current user")]
    UnauthorizedAction,
    #[error("database error: {message}")]
    Database { message: String },
}
