use serde::{Deserialize, Serialize};

use crate::*;

/// A search expression — a normalized text fragment extracted from an entity
/// and indexed for full-text search. Expressions are projection targets:
/// events produce them, and the FTS5 index makes them queryable.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Expression {
    pub resource_ref: Ref,
    pub kind: Label,
    pub content: Content,
}

/// Envelope for search results, pairing the original query with matches.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResults {
    pub query: String,
    pub results: Vec<Expression>,
}
