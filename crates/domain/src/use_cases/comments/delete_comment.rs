//! Delete comment use case

use crate::{
    CommentId, DomainError, DomainResult, UserId,
    repositories::{ArticlesRepository, CommentsRepository},
};

/// Delete a comment
///
/// # Business Rules
/// - Article must exist
/// - Comment must exist and belong to the article
/// - Only the comment author can delete it
pub async fn delete_comment<A, C>(
    articles_repo: &A,
    comments_repo: &C,
    slug: &str,
    comment_id: CommentId,
    user_id: UserId,
) -> DomainResult<()>
where
    A: ArticlesRepository,
    C: CommentsRepository,
{
    // Verify article exists
    let article = articles_repo
        .get_article_by_slug(slug)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    // Get comment
    let comment = comments_repo
        .get_comment_by_id(comment_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "comment" })?
        .ok_or(DomainError::NotFound { entity: "comment" })?;

    // Verify comment belongs to article
    if comment.article_id != article.id {
        return Err(DomainError::NotFound { entity: "comment" });
    }

    // Authorization: only author can delete
    if comment.author_id != user_id {
        return Err(DomainError::NotFound { entity: "comment" });
    }

    comments_repo
        .delete_comment(comment_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "comment" })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryCommentsRepository, InMemoryUsersRepository};
    use crate::{Article, ArticleDraft, ArticleId, Comment, CommentDraft, TagList};
    use chrono::Utc;

    async fn setup() -> (InMemoryArticlesRepository, InMemoryCommentsRepository, Article, Comment, UserId) {
        let users_repo = InMemoryUsersRepository::new();
        let articles_repo = InMemoryArticlesRepository::new(users_repo);
        let comments_repo = InMemoryCommentsRepository::new();
        let author_id = UserId::random();

        let draft = ArticleDraft::new("Test Article", "Description", "Body", TagList::default()).unwrap();
        let article = Article::publish(ArticleId::random(), author_id, draft, Utc::now()).unwrap();
        let article = articles_repo.create_article(article).await.unwrap();

        let comment_draft = CommentDraft::new("Test comment").unwrap();
        let comment = Comment::new(CommentId::new(1), article.id, author_id, comment_draft, Utc::now());
        let comment = comments_repo.create_comment(comment).await.unwrap();

        (articles_repo, comments_repo, article, comment, author_id)
    }

    #[tokio::test]
    async fn test_delete_comment_success() {
        let (articles_repo, comments_repo, article, comment, author_id) = setup().await;

        let result = delete_comment(
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            comment.id,
            author_id,
        )
        .await;

        assert!(result.is_ok());

        // Verify comment is deleted
        let found = comments_repo.get_comment_by_id(comment.id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_comment_article_not_found() {
        let (articles_repo, comments_repo, _, comment, author_id) = setup().await;

        let result = delete_comment(
            &articles_repo,
            &comments_repo,
            "nonexistent",
            comment.id,
            author_id,
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }

    #[tokio::test]
    async fn test_delete_comment_comment_not_found() {
        let (articles_repo, comments_repo, article, _, author_id) = setup().await;

        let result = delete_comment(
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            CommentId::new(999),
            author_id,
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "comment" })));
    }

    #[tokio::test]
    async fn test_delete_comment_unauthorized() {
        let (articles_repo, comments_repo, article, comment, _) = setup().await;
        let other_user_id = UserId::random();

        let result = delete_comment(
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            comment.id,
            other_user_id,
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "comment" })));
    }

    #[tokio::test]
    async fn test_delete_comment_wrong_article() {
        let (articles_repo, comments_repo, _, comment, author_id) = setup().await;

        // Create another article
        let draft = ArticleDraft::new("Other Article", "Description", "Body", TagList::default()).unwrap();
        let other_article = Article::publish(ArticleId::random(), author_id, draft, Utc::now()).unwrap();
        let other_article = articles_repo.create_article(other_article).await.unwrap();

        let result = delete_comment(
            &articles_repo,
            &comments_repo,
            other_article.slug.as_str(),
            comment.id,
            author_id,
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "comment" })));
    }
}
