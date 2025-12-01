//! Use cases / application logic
//!
//! Each use case represents a single business operation and orchestrates
//! domain entities, services, and repositories to accomplish it.
//!
//! Use cases are organized by domain aggregate:
//! - `articles` - Article CRUD, favorites, feed
//! - `users` - Registration, login, profile updates
//! - `profiles` - Follow/unfollow users
//! - `comments` - Article comments

pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

// Re-export all use cases for convenient access
pub use articles::*;
pub use comments::*;
pub use profiles::*;
pub use users::*;

use crate::repositories::{ArticlesRepository, CommentsRepository, UsersRepository};

/// Container for all repositories with injected dependencies
///
/// This struct holds references to all repositories and can be used
/// to access them in a unified way. Use case functions are standalone
/// and receive repository references as parameters.
pub struct UseCases<U, A, C>
where
    U: UsersRepository,
    A: ArticlesRepository,
    C: CommentsRepository,
{
    pub users_repo: U,
    pub articles_repo: A,
    pub comments_repo: C,
}

impl<U, A, C> UseCases<U, A, C>
where
    U: UsersRepository,
    A: ArticlesRepository,
    C: CommentsRepository,
{
    pub fn new(users_repo: U, articles_repo: A, comments_repo: C) -> Self {
        Self {
            users_repo,
            articles_repo,
            comments_repo,
        }
    }
}

impl<U, A, C> Clone for UseCases<U, A, C>
where
    U: UsersRepository + Clone,
    A: ArticlesRepository + Clone,
    C: CommentsRepository + Clone,
{
    fn clone(&self) -> Self {
        Self {
            users_repo: self.users_repo.clone(),
            articles_repo: self.articles_repo.clone(),
            comments_repo: self.comments_repo.clone(),
        }
    }
}
