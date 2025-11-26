use std::{
    collections::HashMap,
    sync::Arc,
};

use domain::{
    Tag, TagList,
    use_cases::UseCases,
    UserId,
    repositories::{UsersRepository, ArticlesRepository, CommentsRepository},
};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState<U = data::PostgresUsersRepository, A = data::PostgresArticlesRepository, C = data::PostgresCommentsRepository>
where
    U: UsersRepository + Clone,
    A: ArticlesRepository + Clone,
    C: CommentsRepository + Clone,
{
    // TODO: Replace with proper JWT-based auth or session store (Redis)
    pub sessions: Arc<RwLock<HashMap<String, UserId>>>,
    pub use_cases: Arc<UseCases<U, A, C>>,
    // TODO: Extract tags from database or maintain as cache
    pub tags: Arc<RwLock<TagList>>,
}

impl AppState {
    pub fn new(use_cases: UseCases<data::PostgresUsersRepository, data::PostgresArticlesRepository, data::PostgresCommentsRepository>) -> Self {
        let mut tags = TagList::default();
        for label in ["rust", "axum", "realworld"] {
            if let Ok(tag) = Tag::new(label) {
                tags.push(tag);
            }
        }
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            use_cases: Arc::new(use_cases),
            tags: Arc::new(RwLock::new(tags)),
        }
    }
}

#[cfg(test)]
impl AppState<domain::repositories::InMemoryUsersRepository, domain::repositories::InMemoryArticlesRepository, domain::repositories::InMemoryCommentsRepository> {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        use domain::repositories::{InMemoryUsersRepository, InMemoryArticlesRepository, InMemoryCommentsRepository};
        
        let users_repo = InMemoryUsersRepository::new();
        let articles_repo = InMemoryArticlesRepository::new(users_repo.clone());
        let comments_repo = InMemoryCommentsRepository::new();
        
        let use_cases = UseCases::new(users_repo, articles_repo, comments_repo);
        
        let mut tags = TagList::default();
        for label in ["rust", "axum", "realworld"] {
            if let Ok(tag) = Tag::new(label) {
                tags.push(tag);
            }
        }
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            use_cases: Arc::new(use_cases),
            tags: Arc::new(RwLock::new(tags)),
        }
    }
}
