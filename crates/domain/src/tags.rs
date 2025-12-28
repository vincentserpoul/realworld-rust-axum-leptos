use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::errors::{DomainError, DomainResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Tag(String);

impl Tag {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let trimmed = value.into().trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidTag);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Tag> for String {
    fn from(value: Tag) -> Self {
        value.0
    }
}

/// A list of tags that serializes as a flat array of strings.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TagList {
    tags: Vec<Tag>,
}

impl Serialize for TagList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as a flat array of strings
        serializer.collect_seq(self.tags.iter().map(|t| t.as_str()))
    }
}

impl<'de> Deserialize<'de> for TagList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strings: Vec<String> = Vec::deserialize(deserializer)?;
        let tags = strings
            .into_iter()
            .filter_map(|s| Tag::new(s).ok())
            .collect();
        Ok(Self { tags })
    }
}

impl TagList {
    pub fn new<I, T>(iter: I) -> DomainResult<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let mut tags = Vec::new();
        for item in iter {
            let tag = Tag::new(item.into())?;
            if tags.iter().any(|existing| existing == &tag) {
                continue;
            }
            tags.push(tag);
        }
        Ok(Self { tags })
    }

    pub fn as_slice(&self) -> &[Tag] {
        &self.tags
    }

    pub fn contains(&self, tag: &Tag) -> bool {
        self.tags.iter().any(|existing| existing == tag)
    }

    pub fn push(&mut self, tag: Tag) {
        if !self.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_validation_rejects_blank_values() {
        assert!(Tag::new("rust").is_ok());
        assert_eq!(Tag::new("   ").unwrap_err(), DomainError::InvalidTag);
    }

    #[test]
    fn tag_list_deduplicates() {
        let tags = TagList::new(["rust", "rust", "axum"]).unwrap();
        assert_eq!(tags.as_slice().len(), 2);
        assert!(tags.contains(&Tag::new("rust").unwrap()));
    }
}
