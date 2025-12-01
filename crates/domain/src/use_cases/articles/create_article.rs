//! Create article use case

use chrono::{DateTime, Utc};

use crate::{
    Article, ArticleDraft, ArticleId, ArticleView, DomainResult, Profile, TagList, UserId,
    repositories::ArticlesRepository,
};

/// Input for creating a new article
#[derive(Debug, Clone)]
pub struct CreateArticleInput {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

/// Create a new article
///
/// # Business Rules
/// - Title, description, and body must not be empty
/// - Slug is generated from title
/// - Author is the current user
/// - Initial favorites count is 0
pub async fn create_article<A>(
    articles_repo: &A,
    author_id: UserId,
    author_profile: Profile,
    input: CreateArticleInput,
    now: DateTime<Utc>,
) -> DomainResult<ArticleView>
where
    A: ArticlesRepository,
{
    let tags = TagList::new(input.tag_list)?;
    let draft = ArticleDraft::new(input.title, input.description, input.body, tags)?;

    let article = Article::publish(ArticleId::random(), author_id, draft, now)?;

    let created = articles_repo
        .create_article(article)
        .await
        .map_err(|e| crate::DomainError::NotFound {
            entity: Box::leak(e.to_string().into_boxed_str()),
        })?;

    Ok(created.to_view(author_profile, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{InMemoryArticlesRepository, InMemoryUsersRepository};
    use crate::user::Username;

    fn author_profile() -> Profile {
        Profile::new(Username::new("author").unwrap(), None, None, false)
    }

    #[tokio::test]
    async fn test_create_article_success() {
        let users_repo = InMemoryUsersRepository::new();
        let repo = InMemoryArticlesRepository::new(users_repo);
        let author_id = UserId::random();
        let input = CreateArticleInput {
            title: "Test Article".to_string(),
            description: "A test description".to_string(),
            body: "The article body".to_string(),
            tag_list: vec!["rust".to_string(), "testing".to_string()],
        };

        let result = create_article(&repo, author_id, author_profile(), input, Utc::now()).await;

        assert!(result.is_ok());
        let view = result.unwrap();
        assert_eq!(view.title, "Test Article");
        assert_eq!(view.slug.as_str(), "test-article");
        assert!(!view.favorited);
        assert_eq!(view.favorites_count, 0);
    }

    #[tokio::test]
    async fn test_create_article_empty_title_fails() {
        let users_repo = InMemoryUsersRepository::new();
        let repo = InMemoryArticlesRepository::new(users_repo);
        let author_id = UserId::random();
        let input = CreateArticleInput {
            title: "   ".to_string(),
            description: "A test description".to_string(),
            body: "The article body".to_string(),
            tag_list: vec![],
        };

        let result = create_article(&repo, author_id, author_profile(), input, Utc::now()).await;

        assert!(result.is_err());
    }
}
