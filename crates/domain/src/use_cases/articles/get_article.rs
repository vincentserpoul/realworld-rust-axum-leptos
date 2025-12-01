//! Get article use case

use crate::{
    Article, ArticleView, DomainError, DomainResult, UserId,
    repositories::{ArticlesRepository, UsersRepository},
};

/// Get an article by slug with author profile and favorite status
///
/// # Business Rules
/// - Article must exist
/// - Author profile includes following status relative to viewer
/// - Favorited status is relative to viewer (false if no viewer)
pub async fn get_article<U, A>(
    users_repo: &U,
    articles_repo: &A,
    slug: &str,
    viewer_id: Option<UserId>,
) -> DomainResult<ArticleView>
where
    U: UsersRepository,
    A: ArticlesRepository,
{
    let article = articles_repo
        .get_article_by_slug(slug)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    build_article_view(users_repo, articles_repo, &article, viewer_id).await
}

/// Get an article by ID with author profile and favorite status
pub async fn get_article_by_id<U, A>(
    users_repo: &U,
    articles_repo: &A,
    article_id: crate::ArticleId,
    viewer_id: Option<UserId>,
) -> DomainResult<ArticleView>
where
    U: UsersRepository,
    A: ArticlesRepository,
{
    let article = articles_repo
        .get_article_by_id(article_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    build_article_view(users_repo, articles_repo, &article, viewer_id).await
}

/// Build an article view with author profile and favorite status
pub(crate) async fn build_article_view<U, A>(
    users_repo: &U,
    articles_repo: &A,
    article: &Article,
    viewer_id: Option<UserId>,
) -> DomainResult<ArticleView>
where
    U: UsersRepository,
    A: ArticlesRepository,
{
    let author = users_repo
        .get_user_by_id(article.author_id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "author" })?
        .ok_or(DomainError::NotFound { entity: "author" })?;

    let following = match viewer_id {
        Some(viewer) => users_repo
            .is_following(viewer, article.author_id)
            .await
            .unwrap_or(false),
        None => false,
    };

    let favorited = match viewer_id {
        Some(viewer) => articles_repo
            .is_favorited(viewer, article.id)
            .await
            .unwrap_or(false),
        None => false,
    };

    let profile = author.to_profile(following);
    Ok(article.to_view(profile, favorited))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryUsersRepository};
    use crate::{ArticleDraft, ArticleId, Email, PasswordHash, TagList, User, Username};
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

        let draft = ArticleDraft::new("Test Article", "Description", "Body", TagList::default()).unwrap();
        let article = Article::publish(ArticleId::random(), author.id, draft, Utc::now()).unwrap();
        let article = articles_repo.create_article(article).await.unwrap();

        (users_repo, articles_repo, author, article)
    }

    #[tokio::test]
    async fn test_get_article_success() {
        let (users_repo, articles_repo, _author, article) = setup().await;

        let result = get_article(&users_repo, &articles_repo, article.slug.as_str(), None).await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.slug.as_str(), article.slug.as_str());
        assert!(!view.favorited);
        assert!(!view.author.following);
    }

    #[tokio::test]
    async fn test_get_article_not_found() {
        let (users_repo, articles_repo, _, _) = setup().await;

        let result = get_article(&users_repo, &articles_repo, "nonexistent", None).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }

    #[tokio::test]
    async fn test_get_article_with_viewer_following() {
        let (users_repo, articles_repo, author, article) = setup().await;
        let viewer_id = UserId::random();

        users_repo.follow_user(viewer_id, author.id).await.unwrap();

        let result = get_article(&users_repo, &articles_repo, article.slug.as_str(), Some(viewer_id)).await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert!(view.author.following);
    }
}
