mod actor;
mod agent;
mod brain;
mod cognition;
mod connection;
mod event_ops;
mod experience;
mod failures;
mod level;
mod lifecycle;
mod memory;
mod nature;
mod persona;
mod pressure;
mod search_ops;
mod sensation;
mod serde_stability_tests;
mod storage;
mod tenant;
mod texture;
mod ticket;
mod urge;

use serde::{Deserialize, Serialize};

use crate::*;

pub use actor::*;
pub use agent::*;
pub use brain::*;
pub use cognition::*;
pub use connection::*;
pub use event_ops::*;
pub use experience::*;
pub use failures::*;
pub use level::*;
pub use lifecycle::*;
pub use memory::*;
pub use nature::*;
pub use persona::*;
pub use pressure::*;
pub use search_ops::*;
pub use sensation::*;
pub use storage::*;
pub use tenant::*;
pub use texture::*;
pub use ticket::*;
pub use urge::*;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Agent(AgentEvents),
    Brain(BrainEvents),
    Cognition(CognitionEvents),
    Connection(ConnectionEvents),
    Dreaming(DreamingEvents),
    Experience(ExperienceEvents),
    Introspecting(IntrospectingEvents),
    Level(LevelEvents),
    Lifecycle(LifecycleEvents),
    Memory(MemoryEvents),
    Nature(NatureEvents),
    Persona(PersonaEvents),
    Reflecting(ReflectingEvents),
    Sensation(SensationEvents),
    Sense(SenseEvents),
    Storage(StorageEvents),
    Tenant(TenantEvents),
    Texture(TextureEvents),
    Ticket(TicketEvents),
    Urge(UrgeEvents),
}

/// Super-enum over all request types. Serde untagged — each inner enum
/// carries its own `{type, data}` tag.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Requests {
    Actor(ActorRequests),
    Agent(AgentRequests),
    Brain(BrainRequests),
    Cognition(CognitionRequests),
    Connection(ConnectionRequests),
    Dreaming(DreamingRequests),
    Event(EventRequests),
    Experience(ExperienceRequests),
    Introspecting(IntrospectingRequests),
    Level(LevelRequests),
    Lifecycle(LifecycleRequests),
    Memory(MemoryRequests),
    Nature(NatureRequests),
    Persona(PersonaRequests),
    Pressure(PressureRequests),
    Reflecting(ReflectingRequests),
    Search(SearchRequests),
    Sensation(SensationRequests),
    Sense(SenseRequests),
    Storage(StorageRequests),
    Tenant(TenantRequests),
    Texture(TextureRequests),
    Ticket(TicketRequests),
    Urge(UrgeRequests),
}

impl From<ActorRequests> for Requests {
    fn from(r: ActorRequests) -> Self {
        Self::Actor(r)
    }
}
impl From<AgentRequests> for Requests {
    fn from(r: AgentRequests) -> Self {
        Self::Agent(r)
    }
}
impl From<BrainRequests> for Requests {
    fn from(r: BrainRequests) -> Self {
        Self::Brain(r)
    }
}
impl From<CognitionRequests> for Requests {
    fn from(r: CognitionRequests) -> Self {
        Self::Cognition(r)
    }
}
impl From<ConnectionRequests> for Requests {
    fn from(r: ConnectionRequests) -> Self {
        Self::Connection(r)
    }
}
impl From<DreamingRequests> for Requests {
    fn from(r: DreamingRequests) -> Self {
        Self::Dreaming(r)
    }
}
impl From<EventRequests> for Requests {
    fn from(r: EventRequests) -> Self {
        Self::Event(r)
    }
}
impl From<ExperienceRequests> for Requests {
    fn from(r: ExperienceRequests) -> Self {
        Self::Experience(r)
    }
}
impl From<IntrospectingRequests> for Requests {
    fn from(r: IntrospectingRequests) -> Self {
        Self::Introspecting(r)
    }
}
impl From<LevelRequests> for Requests {
    fn from(r: LevelRequests) -> Self {
        Self::Level(r)
    }
}
impl From<LifecycleRequests> for Requests {
    fn from(r: LifecycleRequests) -> Self {
        Self::Lifecycle(r)
    }
}
impl From<MemoryRequests> for Requests {
    fn from(r: MemoryRequests) -> Self {
        Self::Memory(r)
    }
}
impl From<NatureRequests> for Requests {
    fn from(r: NatureRequests) -> Self {
        Self::Nature(r)
    }
}
impl From<PersonaRequests> for Requests {
    fn from(r: PersonaRequests) -> Self {
        Self::Persona(r)
    }
}
impl From<PressureRequests> for Requests {
    fn from(r: PressureRequests) -> Self {
        Self::Pressure(r)
    }
}
impl From<ReflectingRequests> for Requests {
    fn from(r: ReflectingRequests) -> Self {
        Self::Reflecting(r)
    }
}
impl From<SearchRequests> for Requests {
    fn from(r: SearchRequests) -> Self {
        Self::Search(r)
    }
}
impl From<SensationRequests> for Requests {
    fn from(r: SensationRequests) -> Self {
        Self::Sensation(r)
    }
}
impl From<SenseRequests> for Requests {
    fn from(r: SenseRequests) -> Self {
        Self::Sense(r)
    }
}
impl From<StorageRequests> for Requests {
    fn from(r: StorageRequests) -> Self {
        Self::Storage(r)
    }
}
impl From<TenantRequests> for Requests {
    fn from(r: TenantRequests) -> Self {
        Self::Tenant(r)
    }
}
impl From<TextureRequests> for Requests {
    fn from(r: TextureRequests) -> Self {
        Self::Texture(r)
    }
}
impl From<TicketRequests> for Requests {
    fn from(r: TicketRequests) -> Self {
        Self::Ticket(r)
    }
}
impl From<UrgeRequests> for Requests {
    fn from(r: UrgeRequests) -> Self {
        Self::Urge(r)
    }
}

/// Super-enum over all response types. Serde untagged — each inner enum
/// carries its own `{type, data}` tag.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum Responses {
    Actor(ActorResponses),
    Agent(AgentResponses),
    Brain(BrainResponses),
    Cognition(CognitionResponses),
    Connection(ConnectionResponses),
    Dreaming(DreamingResponses),
    Event(EventResponses),
    Experience(ExperienceResponses),
    Introspecting(IntrospectingResponses),
    Level(LevelResponses),
    Lifecycle(LifecycleResponses),
    Memory(MemoryResponses),
    Nature(NatureResponses),
    Persona(PersonaResponses),
    Pressure(PressureResponses),
    Reflecting(ReflectingResponses),
    Search(SearchResponses),
    Sensation(SensationResponses),
    Sense(SenseResponses),
    Storage(StorageResponses),
    Tenant(TenantResponses),
    Texture(TextureResponses),
    Ticket(TicketResponses),
    Urge(UrgeResponses),
}

impl From<ActorResponses> for Responses {
    fn from(r: ActorResponses) -> Self {
        Self::Actor(r)
    }
}
impl From<AgentResponses> for Responses {
    fn from(r: AgentResponses) -> Self {
        Self::Agent(r)
    }
}
impl From<BrainResponses> for Responses {
    fn from(r: BrainResponses) -> Self {
        Self::Brain(r)
    }
}
impl From<CognitionResponses> for Responses {
    fn from(r: CognitionResponses) -> Self {
        Self::Cognition(r)
    }
}
impl From<ConnectionResponses> for Responses {
    fn from(r: ConnectionResponses) -> Self {
        Self::Connection(r)
    }
}
impl From<DreamingResponses> for Responses {
    fn from(r: DreamingResponses) -> Self {
        Self::Dreaming(r)
    }
}
impl From<EventResponses> for Responses {
    fn from(r: EventResponses) -> Self {
        Self::Event(r)
    }
}
impl From<ExperienceResponses> for Responses {
    fn from(r: ExperienceResponses) -> Self {
        Self::Experience(r)
    }
}
impl From<IntrospectingResponses> for Responses {
    fn from(r: IntrospectingResponses) -> Self {
        Self::Introspecting(r)
    }
}
impl From<LevelResponses> for Responses {
    fn from(r: LevelResponses) -> Self {
        Self::Level(r)
    }
}
impl From<LifecycleResponses> for Responses {
    fn from(r: LifecycleResponses) -> Self {
        Self::Lifecycle(r)
    }
}
impl From<MemoryResponses> for Responses {
    fn from(r: MemoryResponses) -> Self {
        Self::Memory(r)
    }
}
impl From<NatureResponses> for Responses {
    fn from(r: NatureResponses) -> Self {
        Self::Nature(r)
    }
}
impl From<PersonaResponses> for Responses {
    fn from(r: PersonaResponses) -> Self {
        Self::Persona(r)
    }
}
impl From<PressureResponses> for Responses {
    fn from(r: PressureResponses) -> Self {
        Self::Pressure(r)
    }
}
impl From<ReflectingResponses> for Responses {
    fn from(r: ReflectingResponses) -> Self {
        Self::Reflecting(r)
    }
}
impl From<SearchResponses> for Responses {
    fn from(r: SearchResponses) -> Self {
        Self::Search(r)
    }
}
impl From<SensationResponses> for Responses {
    fn from(r: SensationResponses) -> Self {
        Self::Sensation(r)
    }
}
impl From<SenseResponses> for Responses {
    fn from(r: SenseResponses) -> Self {
        Self::Sense(r)
    }
}
impl From<StorageResponses> for Responses {
    fn from(r: StorageResponses) -> Self {
        Self::Storage(r)
    }
}
impl From<TenantResponses> for Responses {
    fn from(r: TenantResponses) -> Self {
        Self::Tenant(r)
    }
}
impl From<TextureResponses> for Responses {
    fn from(r: TextureResponses) -> Self {
        Self::Texture(r)
    }
}
impl From<TicketResponses> for Responses {
    fn from(r: TicketResponses) -> Self {
        Self::Ticket(r)
    }
}
impl From<UrgeResponses> for Responses {
    fn from(r: UrgeResponses) -> Self {
        Self::Urge(r)
    }
}

// ── Response envelope ─────────────────────────────────────────────

/// Ambient metadata carried alongside every response.
/// Extensible for future cross-cutting concerns.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ResponseMeta {
    pub pressure: Vec<PressureReading>,
}

/// Unified protocol response. Every transport returns this shape.
///
/// Flattening `Responses` produces `{ type, data, meta }` — the domain
/// response's tag and content merge with the optional meta field.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Response {
    #[serde(flatten)]
    pub data: Responses,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,
}

impl Response {
    pub fn new(data: impl Into<Responses>) -> Self {
        Self {
            data: data.into(),
            meta: None,
        }
    }

    pub fn with_meta(mut self, meta: ResponseMeta) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Extract the inner domain data, consuming the response.
    ///
    /// Serializes `Responses` to a JSON value, extracts the `data` field
    /// from the adjacently-tagged inner enum, and deserializes as `T`.
    ///
    /// ```ignore
    /// let agent: Agent = response.data()?;
    /// ```
    pub fn data<T: serde::de::DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        let value = serde_json::to_value(self.data)?;
        if let Some(inner) = value.get("data") {
            serde_json::from_value(inner.clone())
        } else {
            serde_json::from_value(value)
        }
    }

    /// Get pressure readings from the response meta, if present.
    pub fn pressure_readings(&self) -> Vec<PressureReading> {
        self.meta
            .as_ref()
            .map(|m| m.pressure.clone())
            .unwrap_or_default()
    }
}

// ── Agent scope extraction ────────────────────────────────────────

impl Requests {
    /// Returns the agent scope for this request, if one is specified.
    ///
    /// Used by Response assembly to determine which agent's pressure
    /// readings to include in ResponseMeta.
    pub fn agent_scope(&self) -> Option<&AgentName> {
        match self {
            // Agent-scoped: required agent field
            Self::Agent(AgentRequests::CreateAgent(r)) => Some(&r.name),
            Self::Agent(AgentRequests::UpdateAgent(r)) => Some(&r.name),
            Self::Agent(AgentRequests::GetAgent(r)) => Some(&r.name),
            Self::Agent(AgentRequests::RemoveAgent(r)) => Some(&r.name),
            Self::Cognition(CognitionRequests::AddCognition(r)) => Some(&r.agent),
            Self::Memory(MemoryRequests::AddMemory(r)) => Some(&r.agent),
            Self::Experience(ExperienceRequests::CreateExperience(r)) => Some(&r.agent),
            Self::Lifecycle(LifecycleRequests::Wake(r)) => Some(&r.agent),
            Self::Lifecycle(LifecycleRequests::Sleep(r)) => Some(&r.agent),
            Self::Lifecycle(LifecycleRequests::Emerge(r)) => Some(&r.name),
            Self::Lifecycle(LifecycleRequests::Recede(r)) => Some(&r.agent),
            Self::Dreaming(DreamingRequests::Dream(r)) => Some(&r.agent),
            Self::Introspecting(IntrospectingRequests::Introspect(r)) => Some(&r.agent),
            Self::Reflecting(ReflectingRequests::Reflect(r)) => Some(&r.agent),
            Self::Sense(SenseRequests::Sense(r)) => Some(&r.agent),
            Self::Pressure(PressureRequests::GetPressure(r)) => Some(&r.agent),

            // Optionally scoped: use filter if provided
            Self::Cognition(CognitionRequests::ListCognitions(r)) => r.agent.as_ref(),
            Self::Memory(MemoryRequests::ListMemories(r)) => r.agent.as_ref(),
            Self::Experience(ExperienceRequests::ListExperiences(r)) => r.agent.as_ref(),
            Self::Search(SearchRequests::Search(r)) => r.agent.as_ref(),

            // Everything else: brain-scoped, no agent context
            _ => None,
        }
    }
}
