//! Listed — a bounded window into a collection.
//!
//! Index operations return items plus the total count, so the
//! consumer knows how deep the well goes. This is the response
//! side of SearchFilters.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A bounded window of items with the total count.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub(crate) struct Listed<T> {
    pub(crate) items: Vec<T>,
    pub(crate) total: usize,
}

impl<T> Listed<T> {
    pub(crate) fn new(items: Vec<T>, total: usize) -> Self {
        Self { items, total }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    pub(crate) fn map<U>(self, f: impl FnMut(T) -> U) -> Listed<U> {
        Listed {
            items: self.items.into_iter().map(f).collect(),
            total: self.total,
        }
    }
}
