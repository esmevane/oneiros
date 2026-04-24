use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A search expression — a normalized text fragment extracted from an entity
/// and indexed for full-text search. Expressions are projection targets:
/// events produce them, and the FTS5 index makes them queryable.
///
/// Facet fields (`agent`, `texture`, `level`, `sensation`, `persona`,
/// `created_at`) are `Option` because each content-bearing kind populates a
/// different subset. Missing values serialize as absent, not empty strings.
#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Expression {
    pub resource_ref: Ref,
    #[builder(into)]
    pub kind: Label,
    #[builder(into)]
    pub content: Content,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<TextureName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<LevelName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensation: Option<SensationName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persona: Option<PersonaName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// The raw text submitted as a search query.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct QueryText(pub String);

impl QueryText {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for QueryText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// Envelope for search results, pairing the original query with matches
/// and faceted aggregations.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchResults {
    pub query: QueryText,
    pub total: usize,
    pub hits: Vec<Expression>,
    #[serde(default, skip_serializing_if = "Facets::is_empty")]
    pub facets: Facets,
}

/// Ordered collection of faceted aggregations — the palace map.
///
/// Each `FacetGroup` names a dimension (kind, agent, texture, etc.) and
/// carries one bucket per observed value. Groups with no buckets are
/// omitted so callers only see wings of the palace that actually have doors.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Facets(pub Vec<FacetGroup>);

impl Facets {
    pub fn new(groups: Vec<FacetGroup>) -> Self {
        Self(
            groups
                .into_iter()
                .filter(|g| !g.buckets.is_empty())
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find(&self, facet: FacetName) -> Option<&FacetGroup> {
        self.0.iter().find(|g| g.facet == facet)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct FacetGroup {
    pub facet: FacetName,
    pub buckets: Vec<FacetBucket>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct FacetBucket {
    pub value: String,
    pub count: usize,
}

/// The named dimensions a search response can aggregate on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FacetName {
    Kind,
    Agent,
    Texture,
    Level,
    Sensation,
    Persona,
}

impl FacetName {
    pub fn column(self) -> &'static str {
        match self {
            FacetName::Kind => "kind",
            FacetName::Agent => "agent_id",
            FacetName::Texture => "texture",
            FacetName::Level => "level",
            FacetName::Sensation => "sensation",
            FacetName::Persona => "persona",
        }
    }
}

/// The kind of content-bearing entity a search hit refers to.
///
/// Mirrors the `kind` column stored in `search_index`. Used both as a filter
/// on queries and as a label on hits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SearchKind {
    Cognition,
    Memory,
    Experience,
    Agent,
}

impl SearchKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SearchKind::Cognition => "cognition",
            SearchKind::Memory => "memory",
            SearchKind::Experience => "experience",
            SearchKind::Agent => "agent",
        }
    }
}

impl core::fmt::Display for SearchKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::str::FromStr for SearchKind {
    type Err = SearchKindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cognition" => Ok(SearchKind::Cognition),
            "memory" => Ok(SearchKind::Memory),
            "experience" => Ok(SearchKind::Experience),
            "agent" => Ok(SearchKind::Agent),
            other => Err(SearchKindParseError(other.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown search kind: {0}")]
pub struct SearchKindParseError(pub String);
