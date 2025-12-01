//! Feed articles use case

use crate::{
    ArticlesEnvelope, DomainResult, FeedFilters, Pagination, UserId,
    repositories::ArticlesRepository,
};

/// Input for fetching user feed
#[derive(Debug, Clone, Default)]
pub struct FeedArticlesInput {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Get articles from users the current user follows
///
/// # Business Rules
/// - Only returns articles from followed users
/// - Pagination is applied with default limits
/// - Articles are returned in descending order by creation date
pub async fn feed_articles<A>(
    articles_repo: &A,
    user_id: UserId,
    input: FeedArticlesInput,
) -> DomainResult<ArticlesEnvelope>
where
    A: ArticlesRepository,
{
    let pagination = Pagination::new(input.limit, input.offset)?;
    let filters = FeedFilters::new(Some(pagination));

    let envelope = articles_repo
        .feed_articles(user_id, filters)
        .await
        .map_err(|_| crate::DomainError::NotFound { entity: "articles" })?;

    Ok(envelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryUsersRepository};

    #[tokio::test]
    async fn test_feed_articles_empty() {
        let users_repo = InMemoryUsersRepository::new();
        let articles_repo = InMemoryArticlesRepository::new(users_repo);
        let user_id = UserId::random();
        let input = FeedArticlesInput::default();

        let result = feed_articles(&articles_repo, user_id, input).await;

        assert!(result.is_ok());
        let envelope = result.unwrap();
        assert_eq!(envelope.articles_count, 0);
    }
}
