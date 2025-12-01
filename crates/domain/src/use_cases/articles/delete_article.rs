//! Delete article use case

use crate::{
    DomainError, DomainResult, UserId,
    repositories::ArticlesRepository,
};

/// Delete an article by slug
///
/// # Business Rules
/// - Article must exist
/// - Only the author can delete their article
pub async fn delete_article<A>(
    articles_repo: &A,
    slug: &str,
    author_id: UserId,
) -> DomainResult<()>
where
    A: ArticlesRepository,
{
    let article = articles_repo
        .get_article_by_slug(slug)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    // Authorization: only author can delete
    if article.author_id != author_id {
        return Err(DomainError::NotFound { entity: "article" });
    }

    articles_repo
        .delete_article(article.id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryUsersRepository};
    use crate::{Article, ArticleDraft, ArticleId, TagList};
    use chrono::Utc;

    async fn setup() -> (InMemoryArticlesRepository, UserId, Article) {
        let users_repo = InMemoryUsersRepository::new();
        let articles_repo = InMemoryArticlesRepository::new(users_repo);
        let author_id = UserId::random();

        let draft = ArticleDraft::new("Test Article", "Description", "Body", TagList::default()).unwrap();
        let article = Article::publish(ArticleId::random(), author_id, draft, Utc::now()).unwrap();
        let article = articles_repo.create_article(article).await.unwrap();

        (articles_repo, author_id, article)
    }

    #[tokio::test]
    async fn test_delete_article_success() {
        let (articles_repo, author_id, article) = setup().await;

        let result = delete_article(&articles_repo, article.slug.as_str(), author_id).await;

        assert!(result.is_ok());

        // Verify article is deleted
        let found = articles_repo.get_article_by_slug(article.slug.as_str()).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_article_unauthorized() {
        let (articles_repo, _author_id, article) = setup().await;
        let other_user_id = UserId::random();

        let result = delete_article(&articles_repo, article.slug.as_str(), other_user_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }

    #[tokio::test]
    async fn test_delete_article_not_found() {
        let (articles_repo, author_id, _article) = setup().await;

        let result = delete_article(&articles_repo, "nonexistent", author_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }
}
