//! List comments use case

use crate::{
    CommentView, DomainError, DomainResult, UserId,
    repositories::{ArticlesRepository, CommentsRepository, UsersRepository},
};

/// List all comments on an article
///
/// # Business Rules
/// - Article must exist
/// - Comments are sorted by creation date (newest first)
/// - Author profiles include following status relative to viewer
pub async fn list_comments<U, A, C>(
    users_repo: &U,
    articles_repo: &A,
    comments_repo: &C,
    slug: &str,
    viewer_id: Option<UserId>,
) -> DomainResult<Vec<CommentView>>
where
    U: UsersRepository,
    A: ArticlesRepository,
    C: CommentsRepository,
{
    // Verify article exists
    let article = articles_repo
        .get_article_by_slug(slug)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    let mut comments = comments_repo
        .get_comments_by_article(article.id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "comments" })?;

    // Sort by creation date, newest first
    comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut views = Vec::with_capacity(comments.len());
    for comment in comments {
        let author = users_repo
            .get_user_by_id(comment.author_id)
            .await
            .map_err(|_| DomainError::NotFound { entity: "author" })?
            .ok_or(DomainError::NotFound { entity: "author" })?;

        let following = match viewer_id {
            Some(viewer) => users_repo
                .is_following(viewer, author.id)
                .await
                .unwrap_or(false),
            None => false,
        };

        let profile = author.to_profile(following);
        views.push(comment.to_view(profile));
    }

    Ok(views)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryCommentsRepository, InMemoryUsersRepository};
    use crate::{Article, ArticleDraft, ArticleId, Comment, CommentDraft, CommentId, Email, PasswordHash, TagList, User, Username};
    use chrono::Utc;

    async fn setup() -> (
        InMemoryUsersRepository,
        InMemoryArticlesRepository,
        InMemoryCommentsRepository,
        User,
        Article,
    ) {
        let users_repo = InMemoryUsersRepository::new();
        let articles_repo = InMemoryArticlesRepository::new(users_repo.clone());
        let comments_repo = InMemoryCommentsRepository::new();

        let author = User::new(
            UserId::random(),
            Email::parse("author@example.com").unwrap(),
            Username::new("author").unwrap(),
            PasswordHash::new("hash").unwrap(),
            Utc::now(),
        );
        users_repo.create_user(author.clone()).await.unwrap();

        let draft = ArticleDraft::new("Test Article", "Description", "Body", TagList::default()).unwrap();
        let article = Article::publish(ArticleId::random(), author.id, draft, Utc::now()).unwrap();
        let article = articles_repo.create_article(article).await.unwrap();

        (users_repo, articles_repo, comments_repo, author, article)
    }

    #[tokio::test]
    async fn test_list_comments_empty() {
        let (users_repo, articles_repo, comments_repo, _, article) = setup().await;

        let result = list_comments(
            &users_repo,
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            None,
        )
        .await;

        assert!(result.is_ok());
        let views = result.unwrap();
        assert!(views.is_empty());
    }

    #[tokio::test]
    async fn test_list_comments_with_comments() {
        let (users_repo, articles_repo, comments_repo, author, article) = setup().await;

        // Create some comments
        for i in 0..3 {
            let draft = CommentDraft::new(format!("Comment {}", i)).unwrap();
            let comment = Comment::new(CommentId::new(i), article.id, author.id, draft, Utc::now());
            comments_repo.create_comment(comment).await.unwrap();
        }

        let result = list_comments(
            &users_repo,
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            None,
        )
        .await;

        assert!(result.is_ok());
        let views = result.unwrap();
        assert_eq!(views.len(), 3);
    }

    #[tokio::test]
    async fn test_list_comments_article_not_found() {
        let (users_repo, articles_repo, comments_repo, _, _) = setup().await;

        let result = list_comments(
            &users_repo,
            &articles_repo,
            &comments_repo,
            "nonexistent",
            None,
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }
}
