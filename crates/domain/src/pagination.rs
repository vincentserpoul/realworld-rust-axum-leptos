use serde::{Deserialize, Serialize};

use crate::errors::{DomainError, DomainResult};

pub const DEFAULT_LIMIT: u32 = 20;
pub const MAX_LIMIT: u32 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    limit: u32,
    offset: u32,
}

impl Pagination {
    pub fn new(limit: Option<u32>, offset: Option<u32>) -> DomainResult<Self> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT);
        let offset = offset.unwrap_or_default();
        if !(1..=MAX_LIMIT).contains(&limit) {
            return Err(DomainError::LimitOutOfRange);
        }
        if offset > i32::MAX as u32 {
            return Err(DomainError::NegativeOffset);
        }
        Ok(Self { limit, offset })
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: DEFAULT_LIMIT,
            offset: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_applies_defaults() {
        let pagination = Pagination::new(None, None).unwrap();
        assert_eq!(pagination.limit(), DEFAULT_LIMIT);
        assert_eq!(pagination.offset(), 0);
    }

    #[test]
    fn pagination_rejects_out_of_range_limit() {
        assert_eq!(
            Pagination::new(Some(MAX_LIMIT + 1), None).unwrap_err(),
            DomainError::LimitOutOfRange
        );
    }
}
