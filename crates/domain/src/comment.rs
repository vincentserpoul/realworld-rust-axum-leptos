use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, DomainResult};
use crate::identifiers::{ArticleId, CommentId, UserId};
use crate::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDraft {
    pub body: String,
}

impl CommentDraft {
    pub fn new(body: impl Into<String>) -> DomainResult<Self> {
        let body = body.into();
        if body.trim().is_empty() {
            return Err(DomainError::InvalidBody);
        }
        Ok(Self { body })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: CommentId,
    pub article_id: ArticleId,
    pub author_id: UserId,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn new(
        id: CommentId,
        article_id: ArticleId,
        author_id: UserId,
        draft: CommentDraft,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            article_id,
            author_id,
            body: draft.body,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn to_view(&self, author: Profile) -> CommentView {
        CommentView {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            body: self.body.clone(),
            author,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentView {
    pub id: CommentId,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub body: String,
    pub author: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentEnvelope {
    pub comment: CommentView,
}

impl From<CommentView> for CommentEnvelope {
    fn from(value: CommentView) -> Self {
        Self { comment: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsEnvelope {
    pub comments: Vec<CommentView>,
}

impl From<Vec<CommentView>> for CommentsEnvelope {
    fn from(value: Vec<CommentView>) -> Self {
        Self { comments: value }
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

    #[test]
    fn comment_draft_requires_body() {
        assert!(CommentDraft::new("hello").is_ok());
        assert_eq!(
            CommentDraft::new("   ").unwrap_err(),
            DomainError::InvalidBody
        );
    }

    #[test]
    fn comment_to_view_clones_fields() {
        let draft = CommentDraft::new("hello").unwrap();
        let comment = Comment::new(CommentId::new(1), article_id(), user_id(), draft, now());
        let profile = Profile::new(Username::new("jake").unwrap(), None, None, false);
        let view = comment.to_view(profile.clone());
        assert_eq!(view.id.as_i64(), 1);
        assert_eq!(view.body, "hello");
        assert_eq!(view.author.username.as_str(), profile.username.as_str());
    }
}
