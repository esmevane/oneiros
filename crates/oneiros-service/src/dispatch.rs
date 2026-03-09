use oneiros_db::Database;
use oneiros_model::*;
use tokio::sync::broadcast;

use crate::{BrainService, Error, ServiceState};

// ── Brain-scoped dispatch ────────────────────────────────────────────

pub enum BrainDispatch {
    Agent(AgentRequests),
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
    Reflecting(ReflectingRequests),
    Search(SearchRequests),
    Sensation(SensationRequests),
    Sense(SenseRequests),
    Storage(StorageRequests),
    Texture(TextureRequests),
}

pub enum BrainDispatchResponse {
    Agent(AgentResponses),
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
    Reflecting(ReflectingResponses),
    Search(SearchResponses),
    Sensation(SensationResponses),
    Sense(SenseResponses),
    Storage(StorageResponses),
    Texture(TextureResponses),
}

impl From<AgentRequests> for BrainDispatch {
    fn from(r: AgentRequests) -> Self {
        Self::Agent(r)
    }
}
impl From<CognitionRequests> for BrainDispatch {
    fn from(r: CognitionRequests) -> Self {
        Self::Cognition(r)
    }
}
impl From<ConnectionRequests> for BrainDispatch {
    fn from(r: ConnectionRequests) -> Self {
        Self::Connection(r)
    }
}
impl From<DreamingRequests> for BrainDispatch {
    fn from(r: DreamingRequests) -> Self {
        Self::Dreaming(r)
    }
}
impl From<EventRequests> for BrainDispatch {
    fn from(r: EventRequests) -> Self {
        Self::Event(r)
    }
}
impl From<ExperienceRequests> for BrainDispatch {
    fn from(r: ExperienceRequests) -> Self {
        Self::Experience(r)
    }
}
impl From<IntrospectingRequests> for BrainDispatch {
    fn from(r: IntrospectingRequests) -> Self {
        Self::Introspecting(r)
    }
}
impl From<LevelRequests> for BrainDispatch {
    fn from(r: LevelRequests) -> Self {
        Self::Level(r)
    }
}
impl From<LifecycleRequests> for BrainDispatch {
    fn from(r: LifecycleRequests) -> Self {
        Self::Lifecycle(r)
    }
}
impl From<MemoryRequests> for BrainDispatch {
    fn from(r: MemoryRequests) -> Self {
        Self::Memory(r)
    }
}
impl From<NatureRequests> for BrainDispatch {
    fn from(r: NatureRequests) -> Self {
        Self::Nature(r)
    }
}
impl From<PersonaRequests> for BrainDispatch {
    fn from(r: PersonaRequests) -> Self {
        Self::Persona(r)
    }
}
impl From<ReflectingRequests> for BrainDispatch {
    fn from(r: ReflectingRequests) -> Self {
        Self::Reflecting(r)
    }
}
impl From<SearchRequests> for BrainDispatch {
    fn from(r: SearchRequests) -> Self {
        Self::Search(r)
    }
}
impl From<SensationRequests> for BrainDispatch {
    fn from(r: SensationRequests) -> Self {
        Self::Sensation(r)
    }
}
impl From<SenseRequests> for BrainDispatch {
    fn from(r: SenseRequests) -> Self {
        Self::Sense(r)
    }
}
impl From<StorageRequests> for BrainDispatch {
    fn from(r: StorageRequests) -> Self {
        Self::Storage(r)
    }
}
impl From<TextureRequests> for BrainDispatch {
    fn from(r: TextureRequests) -> Self {
        Self::Texture(r)
    }
}

impl From<BrainDispatchResponse> for Responses {
    fn from(r: BrainDispatchResponse) -> Self {
        match r {
            BrainDispatchResponse::Agent(r) => Self::Agent(r),
            BrainDispatchResponse::Cognition(r) => Self::Cognition(r),
            BrainDispatchResponse::Connection(r) => Self::Connection(r),
            BrainDispatchResponse::Dreaming(r) => Self::Dreaming(r),
            BrainDispatchResponse::Event(r) => Self::Event(r),
            BrainDispatchResponse::Experience(r) => Self::Experience(r),
            BrainDispatchResponse::Introspecting(r) => Self::Introspecting(r),
            BrainDispatchResponse::Level(r) => Self::Level(r),
            BrainDispatchResponse::Lifecycle(r) => Self::Lifecycle(r),
            BrainDispatchResponse::Memory(r) => Self::Memory(r),
            BrainDispatchResponse::Nature(r) => Self::Nature(r),
            BrainDispatchResponse::Persona(r) => Self::Persona(r),
            BrainDispatchResponse::Reflecting(r) => Self::Reflecting(r),
            BrainDispatchResponse::Search(r) => Self::Search(r),
            BrainDispatchResponse::Sensation(r) => Self::Sensation(r),
            BrainDispatchResponse::Sense(r) => Self::Sense(r),
            BrainDispatchResponse::Storage(r) => Self::Storage(r),
            BrainDispatchResponse::Texture(r) => Self::Texture(r),
        }
    }
}

// ── System-scoped dispatch ───────────────────────────────────────────

pub enum SystemDispatch {
    Actor(ActorRequests),
    Brain(BrainRequests),
    Tenant(TenantRequests),
    Ticket(TicketRequests),
}

pub enum SystemDispatchResponse {
    Actor(ActorResponses),
    Brain(BrainResponses),
    Tenant(TenantResponses),
    Ticket(TicketResponses),
}

impl From<ActorRequests> for SystemDispatch {
    fn from(r: ActorRequests) -> Self {
        Self::Actor(r)
    }
}
impl From<BrainRequests> for SystemDispatch {
    fn from(r: BrainRequests) -> Self {
        Self::Brain(r)
    }
}
impl From<TenantRequests> for SystemDispatch {
    fn from(r: TenantRequests) -> Self {
        Self::Tenant(r)
    }
}
impl From<TicketRequests> for SystemDispatch {
    fn from(r: TicketRequests) -> Self {
        Self::Ticket(r)
    }
}

impl From<SystemDispatchResponse> for Responses {
    fn from(r: SystemDispatchResponse) -> Self {
        match r {
            SystemDispatchResponse::Actor(r) => Self::Actor(r),
            SystemDispatchResponse::Brain(r) => Self::Brain(r),
            SystemDispatchResponse::Tenant(r) => Self::Tenant(r),
            SystemDispatchResponse::Ticket(r) => Self::Ticket(r),
        }
    }
}

// ── Unified dispatch ─────────────────────────────────────────────────

/// Top-level service dispatcher that routes protocol requests to the
/// appropriate scoped service: brain-scoped or system-scoped.
///
/// Constructed with a reference to `ServiceState` (always available) and
/// optional brain context (available when a brain has been resolved from
/// an auth token).
pub struct OneirosService<'a> {
    state: &'a ServiceState,
    brain_db: Option<&'a Database>,
    event_tx: &'a broadcast::Sender<Event>,
    source: Source,
}

impl<'a> OneirosService<'a> {
    /// Create a system-only dispatcher (no brain context).
    pub fn system(state: &'a ServiceState) -> Self {
        Self {
            state,
            brain_db: None,
            event_tx: state.event_sender(),
            source: state.source(),
        }
    }

    /// Create a full dispatcher with brain context.
    pub fn with_brain(state: &'a ServiceState, brain_db: &'a Database, source: Source) -> Self {
        Self {
            state,
            brain_db: Some(brain_db),
            event_tx: state.event_sender(),
            source,
        }
    }

    /// Dispatch any protocol request to the appropriate service.
    pub fn dispatch(&self, request: impl Into<Requests>) -> Result<Responses, Error> {
        match request.into() {
            // System-scoped
            Requests::Actor(r) => Ok(self.state.system_service()?.dispatch_actor(r)?.into()),
            Requests::Brain(r) => Ok(self.state.system_service()?.dispatch_brain(r)?.into()),
            Requests::Tenant(r) => Ok(self.state.system_service()?.dispatch_tenant(r)?.into()),
            Requests::Ticket(r) => Ok(self.state.system_service()?.dispatch_ticket(r)?.into()),
            // Brain-scoped
            Requests::Agent(r) => Ok(self.brain_service()?.dispatch_agent(r)?.into()),
            Requests::Cognition(r) => Ok(self.brain_service()?.dispatch_cognition(r)?.into()),
            Requests::Connection(r) => Ok(self.brain_service()?.dispatch_connection(r)?.into()),
            Requests::Dreaming(r) => Ok(self.brain_service()?.dispatch_dream(r)?.into()),
            Requests::Event(r) => Ok(self.brain_service()?.dispatch_event(r)?.into()),
            Requests::Experience(r) => Ok(self.brain_service()?.dispatch_experience(r)?.into()),
            Requests::Introspecting(r) => Ok(self.brain_service()?.dispatch_introspect(r)?.into()),
            Requests::Level(r) => Ok(self.brain_service()?.dispatch_level(r)?.into()),
            Requests::Lifecycle(r) => Ok(self.brain_service()?.dispatch_lifecycle(r)?.into()),
            Requests::Memory(r) => Ok(self.brain_service()?.dispatch_memory(r)?.into()),
            Requests::Nature(r) => Ok(self.brain_service()?.dispatch_nature(r)?.into()),
            Requests::Persona(r) => Ok(self.brain_service()?.dispatch_persona(r)?.into()),
            Requests::Reflecting(r) => Ok(self.brain_service()?.dispatch_reflect(r)?.into()),
            Requests::Search(r) => Ok(self.brain_service()?.dispatch_search(r)?.into()),
            Requests::Sensation(r) => Ok(self.brain_service()?.dispatch_sensation(r)?.into()),
            Requests::Sense(r) => Ok(self.brain_service()?.dispatch_sense(r)?.into()),
            Requests::Storage(r) => Ok(self.brain_service()?.dispatch_storage(r)?.into()),
            Requests::Texture(r) => Ok(self.brain_service()?.dispatch_texture(r)?.into()),
        }
    }

    fn brain_service(&self) -> Result<BrainService<'_>, Error> {
        let db = self.brain_db.ok_or(crate::BadRequests::NotHandled(
            "brain-scoped operations require brain context",
        ))?;
        Ok(BrainService::new(db, self.event_tx, self.source))
    }
}
