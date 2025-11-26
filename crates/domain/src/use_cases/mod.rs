/// Use cases / application logic
///
/// Each use case represents a single business operation and orchestrates
/// domain entities, services, and repositories to accomplish it.
use crate::repositories::{ArticlesRepository, CommentsRepository, UsersRepository};

/// Container for all use cases with injected dependencies
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
