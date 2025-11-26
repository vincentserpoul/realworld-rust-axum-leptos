use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::errors::{DomainError, DomainResult};
use crate::identifiers::{ArticleId, UserId};
use crate::pagination::Pagination;
use crate::profile::Profile;
use crate::tags::{Tag, TagList};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Slug(String);

impl Slug {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidSlug);
        }
        Ok(Self(trimmed))
    }

    pub fn from_title(title: &str) -> DomainResult<Self> {
        let mut slug = String::new();
        for ch in title.trim().chars() {
            if ch.is_ascii_alphanumeric() {
                slug.push(ch.to_ascii_lowercase());
            } else if (ch.is_whitespace() || matches!(ch, '-' | '_' | ':'))
                && !slug.ends_with('-')
            {
                slug.push('-');
            }
        }
        let slug = slug.trim_matches('-');
        if slug.is_empty() {
            return Err(DomainError::InvalidSlug);
        }
        Ok(Self(slug.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Slug> for String {
    fn from(value: Slug) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleDraft {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: TagList,
}

impl ArticleDraft {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        body: impl Into<String>,
        tag_list: TagList,
    ) -> DomainResult<Self> {
        let title = title.into();
        let description = description.into();
        let body = body.into();
        if title.trim().is_empty() {
            return Err(DomainError::InvalidTitle);
        }
        if description.trim().is_empty() {
            return Err(DomainError::InvalidDescription);
        }
        if body.trim().is_empty() {
            return Err(DomainError::InvalidBody);
        }
        Ok(Self {
            title,
            description,
            body,
            tag_list,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: ArticleId,
    pub slug: Slug,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: TagList,
    pub author_id: UserId,
    pub favorites_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    pub fn publish(
        id: ArticleId,
        author_id: UserId,
        draft: ArticleDraft,
        now: DateTime<Utc>,
    ) -> DomainResult<Self> {
        let slug = Slug::from_title(&draft.title)?;
        Ok(Self {
            id,
            slug,
            title: draft.title,
            description: draft.description,
            body: draft.body,
            tag_list: draft.tag_list,
            author_id,
            favorites_count: 0,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn apply_changes(
        &mut self,
        changes: ArticleChanges,
        now: DateTime<Utc>,
    ) -> DomainResult<()> {
        if let Some(title) = changes.title {
            if title.trim().is_empty() {
                return Err(DomainError::InvalidTitle);
            }
            self.title = title.to_owned();
            self.slug = Slug::from_title(&self.title)?;
        }
        if let Some(description) = changes.description {
            if description.trim().is_empty() {
                return Err(DomainError::InvalidDescription);
            }
            self.description = description.to_owned();
        }
        if let Some(body) = changes.body {
            if body.trim().is_empty() {
                return Err(DomainError::InvalidBody);
            }
            self.body = body.to_owned();
        }
        if let Some(tags) = changes.tag_list {
            self.tag_list = tags;
        }
        self.updated_at = now;
        Ok(())
    }

    pub fn favorite(&mut self) {
        self.favorites_count = self.favorites_count.saturating_add(1);
    }

    pub fn unfavorite(&mut self) {
        self.favorites_count = self.favorites_count.saturating_sub(1);
    }

    pub fn to_view(&self, author: Profile, favorited: bool) -> ArticleView {
        ArticleView {
            slug: self.slug.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            body: self.body.clone(),
            tag_list: self.tag_list.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited,
            favorites_count: self.favorites_count,
            author,
        }
    }

    pub fn to_summary(&self, author: Profile, favorited: bool) -> ArticleSummary {
        ArticleSummary {
            slug: self.slug.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            tag_list: self.tag_list.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited,
            favorites_count: self.favorites_count,
            author,
        }
    }

    pub fn authored_by(&self, user_id: &UserId) -> bool {
        &self.author_id == user_id
    }

    /// Find article by slug in a collection
    pub fn find_by_slug<'a>(articles: &'a [Article], slug: &str) -> Option<&'a Article> {
        articles.iter().find(|a| a.slug.as_str() == slug)
    }

    /// Find article by slug in a collection (owned)
    pub fn find_by_slug_owned(articles: &[Article], slug: &str) -> Option<Article> {
        articles.iter().find(|a| a.slug.as_str() == slug).cloned()
    }

    /// Create a new article from draft with validation and return view
    pub fn create_from_draft(
        id: ArticleId,
        author_id: UserId,
        draft: ArticleDraft,
        author_profile: Profile,
        now: DateTime<Utc>,
    ) -> DomainResult<(Self, ArticleView)> {
        let article = Self::publish(id, author_id, draft, now)?;
        let view = article.to_view(author_profile, false);
        Ok((article, view))
    }

    /// Build an article view with author profile and favorite status
    pub fn build_view(&self, author: Profile, favorited: bool) -> ArticleView {
        self.to_view(author, favorited)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArticleChanges {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleView {
    pub slug: Slug,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: TagList,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: u32,
    pub author: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSummary {
    pub slug: Slug,
    pub title: String,
    pub description: String,
    pub tag_list: TagList,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: u32,
    pub author: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleList {
    pub articles: Vec<ArticleSummary>,
    #[serde(rename = "articlesCount")]
    pub articles_count: usize,
}

impl ArticleList {
    pub fn new(articles: Vec<ArticleSummary>) -> Self {
        let articles_count = articles.len();
        Self {
            articles,
            articles_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArticleFilters {
    pub tag: Option<Tag>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

impl ArticleFilters {
    pub fn new(
        tag: Option<String>,
        author: Option<String>,
        favorited: Option<String>,
        pagination: Option<Pagination>,
    ) -> DomainResult<Self> {
        let tag = match tag {
            Some(value) => Some(Tag::new(value)?),
            None => None,
        };
        Ok(Self {
            tag,
            author,
            favorited,
            pagination: pagination.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedFilters {
    #[serde(flatten)]
    pub pagination: Pagination,
}

impl FeedFilters {
    pub fn new(pagination: Option<Pagination>) -> Self {
        Self {
            pagination: pagination.unwrap_or_default(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleEnvelope {
    pub article: ArticleView,
}

impl From<ArticleView> for ArticleEnvelope {
    fn from(value: ArticleView) -> Self {
        Self { article: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlesEnvelope {
    pub articles: Vec<ArticleSummary>,
    #[serde(rename = "articlesCount")]
    pub articles_count: usize,
}

impl From<Vec<ArticleSummary>> for ArticlesEnvelope {
    fn from(value: Vec<ArticleSummary>) -> Self {
        Self {
            articles_count: value.len(),
            articles: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::Profile;
    use crate::user::Username;

    fn article_id() -> ArticleId {
        ArticleId::random()
    }

    fn user_id() -> UserId {
        UserId::random()
    }

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn profile(name: &str) -> Profile {
        Profile::new(Username::new(name).unwrap(), None, None, false)
    }

    #[test]
    fn slug_from_title_strips_invalid_chars() {
        let slug = Slug::from_title(" Hello, World! ").unwrap();
        assert_eq!(slug.as_str(), "hello-world");
    }

    #[test]
    fn slug_rejects_empty_titles() {
        assert_eq!(
            Slug::from_title("   ").unwrap_err(),
            DomainError::InvalidSlug
        );
    }

    #[test]
    fn publish_generates_slug_and_defaults_counters() {
        let draft = ArticleDraft::new(
            "How to Train",
            "desc",
            "body",
            TagList::new(["rust", "rust"]).unwrap(),
        )
        .unwrap();
        let article = Article::publish(article_id(), user_id(), draft, now()).unwrap();
        assert_eq!(article.slug.as_str(), "how-to-train");
        assert_eq!(article.favorites_count, 0);
    }

    #[test]
    fn apply_changes_updates_fields_and_slug() {
        let draft = ArticleDraft::new("Title", "desc", "body", TagList::default()).unwrap();
        let mut article = Article::publish(article_id(), user_id(), draft, now()).unwrap();
        let changes = ArticleChanges {
            title: Some("New Title".into()),
            description: Some("new desc".into()),
            body: Some("new body".into()),
            tag_list: Some(TagList::new(["rust"]).unwrap()),
        };
        let future = article.created_at + chrono::Duration::minutes(10);
        article.apply_changes(changes, future).unwrap();
        assert_eq!(article.title, "New Title");
        assert_eq!(article.slug.as_str(), "new-title");
        assert_eq!(article.description, "new desc");
        assert_eq!(article.body, "new body");
        assert_eq!(article.tag_list.as_slice().len(), 1);
        assert_eq!(article.updated_at, future);
    }

    #[test]
    fn favorite_counters_saturate() {
        let draft = ArticleDraft::new("Title", "desc", "body", TagList::default()).unwrap();
        let mut article = Article::publish(article_id(), user_id(), draft, now()).unwrap();
        article.favorite();
        assert_eq!(article.favorites_count, 1);
        article.unfavorite();
        assert_eq!(article.favorites_count, 0);
        article.unfavorite();
        assert_eq!(article.favorites_count, 0);
    }

    #[test]
    fn to_view_clones_all_visible_fields() {
        let draft = ArticleDraft::new("Title", "desc", "body", TagList::default()).unwrap();
        let article = Article::publish(article_id(), user_id(), draft, now()).unwrap();
        let author = profile("alice");
        let view = article.to_view(author.clone(), true);
        assert_eq!(view.slug.as_str(), article.slug.as_str());
        assert_eq!(view.author.username.as_str(), author.username.as_str());
        assert!(view.favorited);
    }
}
