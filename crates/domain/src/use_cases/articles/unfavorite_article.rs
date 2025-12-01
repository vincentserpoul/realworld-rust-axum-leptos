//! Unfavorite article use case

use crate::{
    ArticleView, DomainError, DomainResult, UserId,
    repositories::{ArticlesRepository, UsersRepository},
    use_cases::articles::get_article::build_article_view,
};

/// Unfavorite an article
///
/// # Business Rules
/// - Article must exist
/// - Unfavoriting not-favorited article is idempotent
pub async fn unfavorite_article<U, A>(
    users_repo: &U,
    articles_repo: &A,
    slug: &str,
    user_id: UserId,
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

    articles_repo
        .unfavorite_article(user_id, article.id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?;

    // Re-fetch to get updated favorites_count
    let updated = articles_repo
        .get_article_by_id(article.id)
        .await
        .map_err(|_| DomainError::NotFound { entity: "article" })?
        .ok_or(DomainError::NotFound { entity: "article" })?;

    build_article_view(users_repo, articles_repo, &updated, Some(user_id)).await
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

        let draft = ArticleDraft::new("Test Article", "Description", "Body", TagList::default()).unwrap();
        let article = Article::publish(ArticleId::random(), author.id, draft, Utc::now()).unwrap();
        let article = articles_repo.create_article(article).await.unwrap();

        (users_repo, articles_repo, author, article)
    }

    #[tokio::test]
    async fn test_unfavorite_article_success() {
        let (users_repo, articles_repo, _author, article) = setup().await;
        let user_id = UserId::random();

        // First favorite
        articles_repo.favorite_article(user_id, article.id).await.unwrap();

        // Then unfavorite
        let result = unfavorite_article(&users_repo, &articles_repo, article.slug.as_str(), user_id).await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert!(!view.favorited);
        assert_eq!(view.favorites_count, 0);
    }

    #[tokio::test]
    async fn test_unfavorite_article_not_found() {
        let (users_repo, articles_repo, _, _) = setup().await;
        let user_id = UserId::random();

        let result = unfavorite_article(&users_repo, &articles_repo, "nonexistent", user_id).await;

        assert!(matches!(result, Err(DomainError::NotFound { entity: "article" })));
    }
}
