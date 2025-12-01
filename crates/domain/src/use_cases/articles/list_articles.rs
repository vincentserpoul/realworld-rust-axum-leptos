//! List articles use case

use crate::{
    ArticleFilters, ArticlesEnvelope, DomainResult, Pagination,
    repositories::ArticlesRepository,
};

/// Input for listing articles
#[derive(Debug, Clone, Default)]
pub struct ListArticlesInput {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// List articles with optional filters
///
/// # Business Rules
/// - Pagination is applied with default limits
/// - Can filter by tag, author username, or favorited by username
/// - Articles are returned in descending order by creation date
pub async fn list_articles<A>(
    articles_repo: &A,
    input: ListArticlesInput,
) -> DomainResult<ArticlesEnvelope>
where
    A: ArticlesRepository,
{
    let pagination = Pagination::new(input.limit, input.offset)?;
    let filters = ArticleFilters::new(input.tag, input.author, input.favorited, Some(pagination))?;

    let envelope = articles_repo
        .list_articles(filters)
        .await
        .map_err(|_| crate::DomainError::NotFound { entity: "articles" })?;

    Ok(envelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryUsersRepository, UsersRepository};
    use crate::{Article, ArticleDraft, ArticleId, Email, PasswordHash, TagList, User, UserId, Username};
    use chrono::Utc;

    async fn setup_with_articles() -> InMemoryArticlesRepository {
        let users_repo = InMemoryUsersRepository::new();
        let author_id = UserId::random();
        
        // Create the author user
        let author = User::new(
            author_id,
            Email::parse("author@example.com").unwrap(),
            Username::new("author").unwrap(),
            PasswordHash::new("hash").unwrap(),
            Utc::now(),
        );
        users_repo.create_user(author).await.unwrap();
        
        let articles_repo = InMemoryArticlesRepository::new(users_repo);

        for i in 0..5 {
            let draft = ArticleDraft::new(
                format!("Article {}", i),
                "Description",
                "Body",
                TagList::new(["rust"]).unwrap(),
            )
            .unwrap();
            let article = Article::publish(ArticleId::random(), author_id, draft, Utc::now()).unwrap();
            articles_repo.create_article(article).await.unwrap();
        }

        articles_repo
    }

    #[tokio::test]
    async fn test_list_articles_success() {
        let articles_repo = setup_with_articles().await;
        let input = ListArticlesInput::default();

        let result = list_articles(&articles_repo, input).await;

        assert!(result.is_ok());
        let envelope = result.unwrap();
        assert_eq!(envelope.articles_count, 5);
    }

    #[tokio::test]
    async fn test_list_articles_with_pagination() {
        let articles_repo = setup_with_articles().await;
        let input = ListArticlesInput {
            limit: Some(2),
            offset: Some(0),
            ..Default::default()
        };

        let result = list_articles(&articles_repo, input).await;

        assert!(result.is_ok());
        let envelope = result.unwrap();
        assert_eq!(envelope.articles.len(), 2);
    }
}
