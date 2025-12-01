//! Create comment use case

use chrono::{DateTime, Utc};

use crate::{
    Comment, CommentDraft, CommentId, CommentView, DomainError, DomainResult, UserId,
    repositories::{ArticlesRepository, CommentsRepository, UsersRepository},
};

/// Input for creating a new comment
#[derive(Debug, Clone)]
pub struct CreateCommentInput {
    pub body: String,
}

/// Create a new comment on an article
///
/// # Business Rules
/// - Article must exist
/// - Body must not be empty
/// - Author is the current user
pub async fn create_comment<U, A, C>(
    users_repo: &U,
    articles_repo: &A,
    comments_repo: &C,
    slug: &str,
    author_id: UserId,
    input: CreateCommentInput,
    now: DateTime<Utc>,
) -> DomainResult<CommentView>
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

    // Get author for profile
    let author = users_repo
        .get_user_by_id(author_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "author" })?
        .ok_or(DomainError::NotFound { entity: "author" })?;

    let draft = CommentDraft::new(input.body)?;
    let comment = Comment::new(
        CommentId::new(0), // DB will generate proper ID
        article.id,
        author_id,
        draft,
        now,
    );

    let created = comments_repo
        .create_comment(comment)
        .await
        .map_err(|_| DomainError::NotFound { entity: "comment" })?;

    let profile = author.to_profile(false);
    Ok(created.to_view(profile))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryCommentsRepository, InMemoryUsersRepository};
    use crate::{Article, ArticleDraft, ArticleId, Email, PasswordHash, TagList, User, Username};
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
    async fn test_create_comment_success() {
        let (users_repo, articles_repo, comments_repo, author, article) = setup().await;
        let input = CreateCommentInput {
            body: "Great article!".to_string(),
        };

        let result = create_comment(
            &users_repo,
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            author.id,
            input,
            Utc::now(),
        )
        .await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.body, "Great article!");
        assert_eq!(view.author.username.as_str(), "author");
    }

    #[tokio::test]
    async fn test_create_comment_article_not_found() {
        let (users_repo, articles_repo, comments_repo, author, _) = setup().await;
        let input = CreateCommentInput {
            body: "Comment".to_string(),
        };

        let result = create_comment(
            &users_repo,
            &articles_repo,
            &comments_repo,
            "nonexistent",
            author.id,
            input,
            Utc::now(),
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }

    #[tokio::test]
    async fn test_create_comment_empty_body() {
        let (users_repo, articles_repo, comments_repo, author, article) = setup().await;
        let input = CreateCommentInput {
            body: "   ".to_string(),
        };

        let result = create_comment(
            &users_repo,
            &articles_repo,
            &comments_repo,
            article.slug.as_str(),
            author.id,
            input,
            Utc::now(),
        )
        .await;

        assert!(matches!(result, Err(DomainError::InvalidBody)));
    }
}
