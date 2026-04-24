use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A hydrated search hit — the full domain object behind a `search_index` row.
///
/// One variant per content-bearing kind. Search returns these in FTS5
/// relevance order across kinds; callers that want kind-grouping do so at
/// presentation time. Lists carry their kind-homogeneous slice through the
/// usual domain response (e.g. `CognitionsResponse::items`), built by
/// extracting the matching variant from a search result.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "kind", content = "data", rename_all = "kebab-case")]
pub enum Hit {
    Cognition(Cognition),
    Memory(Memory),
    Experience(Experience),
    Agent(Agent),
}

impl Hit {
    pub fn kind(&self) -> SearchKind {
        match self {
            Self::Cognition(_) => SearchKind::Cognition,
            Self::Memory(_) => SearchKind::Memory,
            Self::Experience(_) => SearchKind::Experience,
            Self::Agent(_) => SearchKind::Agent,
        }
    }

    pub fn resource_ref(&self) -> Ref {
        match self {
            Self::Cognition(c) => Ref::cognition(c.id),
            Self::Memory(m) => Ref::memory(m.id),
            Self::Experience(e) => Ref::experience(e.id),
            Self::Agent(a) => Ref::agent(a.id),
        }
    }

    pub fn content(&self) -> Content {
        match self {
            Self::Cognition(c) => c.content.clone(),
            Self::Memory(m) => m.content.clone(),
            Self::Experience(e) => Content::new(e.description.to_string()),
            Self::Agent(a) => Content::new(format!("{} {}", a.name, a.description)),
        }
    }
}

/// Flat columns for a `search_index` row. Built from a domain object on
/// the write side, decoded into a [`RankedHit`] on the read side.
///
/// Existing `IndexEntry::cognition` etc. constructors normalize the
/// kind-specific facet (texture/level/sensation/persona) into the right
/// column; absent dimensions are empty strings.
pub(crate) struct IndexEntry {
    pub resource_ref: Ref,
    pub kind: SearchKind,
    pub content: Content,
    pub agent_id: AgentId,
    pub texture: Option<TextureName>,
    pub level: Option<LevelName>,
    pub sensation: Option<SensationName>,
    pub persona: Option<PersonaName>,
    pub created_at: Option<Timestamp>,
}

impl IndexEntry {
    pub fn cognition(cognition: &Cognition) -> Self {
        Self {
            resource_ref: Ref::cognition(cognition.id),
            kind: SearchKind::Cognition,
            content: cognition.content.clone(),
            agent_id: cognition.agent_id,
            texture: Some(cognition.texture.clone()),
            level: None,
            sensation: None,
            persona: None,
            created_at: Some(cognition.created_at),
        }
    }

    pub fn memory(memory: &Memory) -> Self {
        Self {
            resource_ref: Ref::memory(memory.id),
            kind: SearchKind::Memory,
            content: memory.content.clone(),
            agent_id: memory.agent_id,
            texture: None,
            level: Some(memory.level.clone()),
            sensation: None,
            persona: None,
            created_at: Some(memory.created_at),
        }
    }

    pub fn experience(experience: &Experience) -> Self {
        Self {
            resource_ref: Ref::experience(experience.id),
            kind: SearchKind::Experience,
            content: Content::new(experience.description.to_string()),
            agent_id: experience.agent_id,
            texture: None,
            level: None,
            sensation: Some(experience.sensation.clone()),
            persona: None,
            created_at: Some(experience.created_at),
        }
    }

    pub fn agent(agent: &Agent) -> Self {
        Self {
            resource_ref: Ref::agent(agent.id),
            kind: SearchKind::Agent,
            content: Content::new(format!("{} {}", agent.name, agent.description)),
            agent_id: agent.id,
            texture: None,
            level: None,
            sensation: None,
            persona: Some(agent.persona.clone()),
            created_at: None,
        }
    }
}

/// A ranked hit pulled from `search_index` — just enough to drive
/// kind-aware hydration through per-domain `get_many` calls. The row
/// preserves FTS5 rank order (or `created_at desc` when no query is
/// present); hydration must reassemble [`Hit`]s in this order. The
/// kind is carried by the `Ref::V0(Resource::...)` variant — no
/// separate discriminator needed.
#[derive(Debug, Clone)]
pub(crate) struct RankedHit {
    pub resource_ref: Ref,
}

impl RankedHit {
    pub(crate) fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let resource_ref = decode_ref(row.get::<_, String>(0)?, 0)?;
        Ok(Self { resource_ref })
    }
}

fn decode_ref(raw: String, col: usize) -> rusqlite::Result<Ref> {
    serde_json::from_str(&raw).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(col, rusqlite::types::Type::Text, Box::new(e))
    })
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
    pub hits: Vec<Hit>,
    #[serde(default, skip_serializing_if = "Facets::is_empty")]
    pub facets: Facets,
}

/// Internal repo-layer result — ranked refs awaiting per-domain
/// hydration. Service layer turns this into [`SearchResults`].
#[derive(Debug)]
pub(crate) struct SearchHits {
    pub total: usize,
    pub hits: Vec<RankedHit>,
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
