use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArticleId(Uuid);

impl ArticleId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for ArticleId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ArticleId> for Uuid {
    fn from(value: ArticleId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommentId(i64);

impl CommentId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for CommentId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<CommentId> for i64 {
    fn from(value: CommentId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_wraps_uuid() {
        let uuid = Uuid::new_v4();
        let id = UserId::from(uuid);
        assert_eq!(id.as_uuid(), uuid);
    }

    #[test]
    fn comment_id_converts_to_i64() {
        let id = CommentId::new(42);
        assert_eq!(id.as_i64(), 42);
        let n: i64 = id.into();
        assert_eq!(n, 42);
    }
}
