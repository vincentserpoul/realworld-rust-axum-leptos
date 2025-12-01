//! Update article use case

use chrono::{DateTime, Utc};

use crate::{
    ArticleChanges, ArticleView, DomainError, DomainResult, TagList, UserId,
    repositories::{ArticlesRepository, UsersRepository},
};

/// Input for updating an article
#[derive(Debug, Clone, Default)]
pub struct UpdateArticleInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

/// Update an existing article
///
/// # Business Rules
/// - Article must exist
/// - Only the author can update their article
/// - Slug is regenerated if title changes
/// - Updated fields must pass validation
pub async fn update_article<U, A>(
    users_repo: &U,
    articles_repo: &A,
    slug: &str,
    author_id: UserId,
    input: UpdateArticleInput,
    now: DateTime<Utc>,
) -> DomainResult<ArticleView>
where
    U: UsersRepository,
    A: ArticlesRepository,
{
    let mut article = articles_repo
        .get_article_by_slug(slug)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    // Authorization: only author can update
    if article.author_id != author_id {
        return Err(DomainError::UnauthorizedAction);
    }

    // Build changes
    let tag_list = match input.tag_list {
        Some(tags) => Some(TagList::new(tags)?),
        None => None,
    };

    let changes = ArticleChanges {
        title: input.title,
        description: input.description,
        body: input.body,
        tag_list,
    };

    article.apply_changes(changes, now)?;

    let updated = articles_repo
        .update_article(article)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?;

    // Build view
    let author = users_repo
        .get_user_by_id(author_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "author" })?
        .ok_or(DomainError::NotFound { entity: "author" })?;

    let favorited = articles_repo
        .is_favorited(author_id, updated.id)
        .await
        .unwrap_or(false);

    let profile = author.to_profile(false);
    Ok(updated.to_view(profile, favorited))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryUsersRepository};
    use crate::{Article, ArticleDraft, ArticleId, Email, PasswordHash, TagList, User, Username};
    use chrono::Utc;

    async fn setup() -> (InMemoryUsersRepository, InMemoryArticlesRepository, User, Article) {
        let users_repo = InMemoryUsersRepository::new();
        let articles_repo = InMemoryArticlesRepository::new(users_repo.clone());

        let author = User::new(
            UserId::random(),
            Email::parse("author@example.com").unwrap(),
            Username::new("author").unwrap(),
            PasswordHash::new("hash").unwrap(),
            Utc::now(),
        );
        users_repo.create_user(author.clone()).await.unwrap();

        let draft = ArticleDraft::new("Original Title", "Description", "Body", TagList::default()).unwrap();
        let article = Article::publish(ArticleId::random(), author.id, draft, Utc::now()).unwrap();
        let article = articles_repo.create_article(article).await.unwrap();

        (users_repo, articles_repo, author, article)
    }

    #[tokio::test]
    async fn test_update_article_success() {
        let (users_repo, articles_repo, author, article) = setup().await;
        let input = UpdateArticleInput {
            title: Some("Updated Title".to_string()),
            ..Default::default()
        };

        let result = update_article(
            &users_repo,
            &articles_repo,
            article.slug.as_str(),
            author.id,
            input,
            Utc::now(),
        )
        .await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.title, "Updated Title");
        assert_eq!(view.slug.as_str(), "updated-title");
    }

    #[tokio::test]
    async fn test_update_article_unauthorized() {
        let (users_repo, articles_repo, _author, article) = setup().await;
        let other_user_id = UserId::random();
        let input = UpdateArticleInput {
            title: Some("Hacked Title".to_string()),
            ..Default::default()
        };

        let result = update_article(
            &users_repo,
            &articles_repo,
            article.slug.as_str(),
            other_user_id,
            input,
            Utc::now(),
        )
        .await;

        assert!(matches!(result, Err(DomainError::UnauthorizedAction)));
    }

    #[tokio::test]
    async fn test_update_article_not_found() {
        let (users_repo, articles_repo, author, _article) = setup().await;
        let input = UpdateArticleInput::default();

        let result = update_article(
            &users_repo,
            &articles_repo,
            "nonexistent",
            author.id,
            input,
            Utc::now(),
        )
        .await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }
}
