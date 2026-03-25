use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A search expression — a normalized text fragment extracted from an entity
/// and indexed for full-text search. Expressions are projection targets:
/// events produce them, and the FTS5 index makes them queryable.
#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Expression {
    pub resource_ref: Ref,
    #[builder(into)]
    pub kind: Label,
    #[builder(into)]
    pub content: Content,
}

/// The raw text submitted as a search query.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct QueryText(pub String);

impl QueryText {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl core::fmt::Display for QueryText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// Envelope for search results, pairing the original query with matches.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResults {
    pub query: QueryText,
    pub results: Vec<Expression>,
}
