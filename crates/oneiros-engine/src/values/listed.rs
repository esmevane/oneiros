//! Listed — a bounded window into a collection.
//!
//! Index operations return items plus the total count, so the
//! consumer knows how deep the well goes. This is the response
//! side of SearchFilters.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A bounded window of items with the total count.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Listed<T> {
    pub items: Vec<T>,
    pub total: usize,
}

impl<T> Listed<T> {
    pub fn new(items: Vec<T>, total: usize) -> Self {
        Self { items, total }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
