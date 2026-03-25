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

/// Envelope for search results, pairing the original query with matches.
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResults {
    pub query: String,
    pub results: Vec<Expression>,
}
