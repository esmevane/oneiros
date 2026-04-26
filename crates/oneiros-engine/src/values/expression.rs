use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A search expression — a normalized text fragment extracted from an entity
/// and indexed for full-text search.
///
/// One variant per content-bearing kind. Each variant wraps a private inner
/// struct so callers can match on kind but cannot destructure the contents
/// — all access goes through the methods on `Expression` itself.
/// Construction goes through the named constructors
/// ([`Expression::cognition`], [`Expression::memory`], etc.); reconstitution
/// from the FTS5 index goes through [`Expression::from_row`].
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Expression {
    Cognition(CognitionExpression),
    Memory(MemoryExpression),
    Experience(ExperienceExpression),
    Agent(AgentExpression),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct CognitionExpression {
    resource_ref: Ref,
    content: Content,
    agent: AgentId,
    texture: TextureName,
    created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct MemoryExpression {
    resource_ref: Ref,
    content: Content,
    agent: AgentId,
    level: LevelName,
    created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ExperienceExpression {
    resource_ref: Ref,
    content: Content,
    agent: AgentId,
    sensation: SensationName,
    created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AgentExpression {
    resource_ref: Ref,
    content: Content,
    agent: AgentId,
    persona: PersonaName,
}

impl Expression {
    pub fn cognition(cognition: &Cognition) -> Self {
        Self::Cognition(CognitionExpression {
            resource_ref: Ref::cognition(cognition.id),
            content: cognition.content.clone(),
            agent: cognition.agent_id,
            texture: cognition.texture.clone(),
            created_at: cognition.created_at,
        })
    }

    pub fn memory(memory: &Memory) -> Self {
        Self::Memory(MemoryExpression {
            resource_ref: Ref::memory(memory.id),
            content: memory.content.clone(),
            agent: memory.agent_id,
            level: memory.level.clone(),
            created_at: memory.created_at,
        })
    }

    pub fn experience(experience: &Experience) -> Self {
        Self::Experience(ExperienceExpression {
            resource_ref: Ref::experience(experience.id),
            content: Content::new(experience.description.to_string()),
            agent: experience.agent_id,
            sensation: experience.sensation.clone(),
            created_at: experience.created_at,
        })
    }

    pub fn agent(agent: &Agent) -> Self {
        Self::Agent(AgentExpression {
            resource_ref: Ref::agent(agent.id),
            content: Content::new(format!("{} {}", agent.name, agent.description)),
            agent: agent.id,
            persona: agent.persona.clone(),
        })
    }

    pub fn kind(&self) -> SearchKind {
        match self {
            Self::Cognition(_) => SearchKind::Cognition,
            Self::Memory(_) => SearchKind::Memory,
            Self::Experience(_) => SearchKind::Experience,
            Self::Agent(_) => SearchKind::Agent,
        }
    }

    pub fn resource_ref(&self) -> &Ref {
        match self {
            Self::Cognition(c) => &c.resource_ref,
            Self::Memory(m) => &m.resource_ref,
            Self::Experience(e) => &e.resource_ref,
            Self::Agent(a) => &a.resource_ref,
        }
    }

    pub fn content(&self) -> &Content {
        match self {
            Self::Cognition(c) => &c.content,
            Self::Memory(m) => &m.content,
            Self::Experience(e) => &e.content,
            Self::Agent(a) => &a.content,
        }
    }

    pub fn agent_id(&self) -> AgentId {
        match self {
            Self::Cognition(c) => c.agent,
            Self::Memory(m) => m.agent,
            Self::Experience(e) => e.agent,
            Self::Agent(a) => a.agent,
        }
    }

    pub fn texture(&self) -> Option<&TextureName> {
        match self {
            Self::Cognition(c) => Some(&c.texture),
            _ => None,
        }
    }

    pub fn level(&self) -> Option<&LevelName> {
        match self {
            Self::Memory(m) => Some(&m.level),
            _ => None,
        }
    }

    pub fn sensation(&self) -> Option<&SensationName> {
        match self {
            Self::Experience(e) => Some(&e.sensation),
            _ => None,
        }
    }

    pub fn persona(&self) -> Option<&PersonaName> {
        match self {
            Self::Agent(a) => Some(&a.persona),
            _ => None,
        }
    }

    pub fn created_at(&self) -> Option<&Timestamp> {
        match self {
            Self::Cognition(c) => Some(&c.created_at),
            Self::Memory(m) => Some(&m.created_at),
            Self::Experience(e) => Some(&e.created_at),
            Self::Agent(_) => None,
        }
    }

    /// Flat column representation matching the `search_index` FTS5 schema.
    /// Absent dimensions render as empty strings — the index column type.
    pub(crate) fn columns(&self) -> ExpressionColumns {
        let resource_ref = serde_json::to_string(self.resource_ref())
            .expect("Ref serializes to JSON without error");
        let content = self.content().to_string();
        let agent = self.agent_id().to_string();
        let texture = self.texture().map(ToString::to_string).unwrap_or_default();
        let level = self.level().map(ToString::to_string).unwrap_or_default();
        let sensation = self
            .sensation()
            .map(ToString::to_string)
            .unwrap_or_default();
        let persona = self.persona().map(ToString::to_string).unwrap_or_default();
        let created_at = self
            .created_at()
            .map(Timestamp::as_string)
            .unwrap_or_default();
        ExpressionColumns {
            resource_ref,
            kind: self.kind(),
            content,
            agent,
            texture,
            level,
            sensation,
            persona,
            created_at,
        }
    }

    /// Reconstitute an expression from a `search_index` row. The row layout
    /// must match `index_expression`'s INSERT order: `resource_ref, kind,
    /// content, agent_id, texture, level, sensation, persona, created_at`.
    pub(crate) fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let resource_ref = decode_ref(row.get::<_, String>(0)?, 0)?;
        let kind: SearchKind = parse_column(row.get::<_, String>(1)?, 1)?;
        let content = Content::new(row.get::<_, String>(2)?);
        let agent: AgentId = parse_column(row.get::<_, String>(3)?, 3)?;

        match kind {
            SearchKind::Cognition => Ok(Self::Cognition(CognitionExpression {
                resource_ref,
                content,
                agent,
                texture: parse_column(row.get::<_, String>(4)?, 4)?,
                created_at: parse_timestamp(row.get::<_, String>(8)?, 8)?,
            })),
            SearchKind::Memory => Ok(Self::Memory(MemoryExpression {
                resource_ref,
                content,
                agent,
                level: parse_column(row.get::<_, String>(5)?, 5)?,
                created_at: parse_timestamp(row.get::<_, String>(8)?, 8)?,
            })),
            SearchKind::Experience => Ok(Self::Experience(ExperienceExpression {
                resource_ref,
                content,
                agent,
                sensation: parse_column(row.get::<_, String>(6)?, 6)?,
                created_at: parse_timestamp(row.get::<_, String>(8)?, 8)?,
            })),
            SearchKind::Agent => Ok(Self::Agent(AgentExpression {
                resource_ref,
                content,
                agent,
                persona: parse_column(row.get::<_, String>(7)?, 7)?,
            })),
        }
    }
}

pub(crate) struct ExpressionColumns {
    pub resource_ref: String,
    pub kind: SearchKind,
    pub content: String,
    pub agent: String,
    pub texture: String,
    pub level: String,
    pub sensation: String,
    pub persona: String,
    pub created_at: String,
}

fn decode_ref(raw: String, col: usize) -> rusqlite::Result<Ref> {
    serde_json::from_str(&raw).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(col, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn parse_column<T>(raw: String, col: usize) -> rusqlite::Result<T>
where
    T: core::str::FromStr,
    T::Err: core::error::Error + Send + Sync + 'static,
{
    raw.parse().map_err(|e: T::Err| {
        rusqlite::Error::FromSqlConversionFailure(col, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn parse_timestamp(raw: String, col: usize) -> rusqlite::Result<Timestamp> {
    Timestamp::parse_str(&raw).map_err(|e| {
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
