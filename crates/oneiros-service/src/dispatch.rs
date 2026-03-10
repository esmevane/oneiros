use std::sync::{Arc, Mutex};

use oneiros_db::Database;
use oneiros_model::*;

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
    Urge(UrgeRequests),
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
    Urge(UrgeResponses),
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
impl From<UrgeRequests> for BrainDispatch {
    fn from(r: UrgeRequests) -> Self {
        Self::Urge(r)
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
            BrainDispatchResponse::Urge(r) => Self::Urge(r),
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

// ── Brain state ─────────────────────────────────────────────────────

/// Owned brain context — wraps a brain database behind a mutex alongside
/// the shared service state. Provides `dispatch` for brain-scoped
/// operations with the lock scoped to a single call.
#[derive(Clone)]
pub struct BrainState {
    state: Arc<ServiceState>,
    database: Arc<Mutex<Database>>,
}

impl BrainState {
    pub fn new(state: Arc<ServiceState>, database: Database) -> Self {
        Self {
            state,
            database: Arc::new(Mutex::new(database)),
        }
    }

    /// Dispatch a brain-scoped request. Locks the database for the
    /// duration of the call.
    pub fn dispatch(
        &self,
        request: impl Into<BrainDispatch>,
    ) -> Result<BrainDispatchResponse, Error> {
        let db = self.database.lock().map_err(|_| Error::DatabasePoisoned)?;
        let service = BrainService::new(&db, self.state.event_sender(), self.state.source());

        match request.into() {
            BrainDispatch::Agent(r) => Ok(BrainDispatchResponse::Agent(service.dispatch_agent(r)?)),
            BrainDispatch::Cognition(r) => Ok(BrainDispatchResponse::Cognition(
                service.dispatch_cognition(r)?,
            )),
            BrainDispatch::Connection(r) => Ok(BrainDispatchResponse::Connection(
                service.dispatch_connection(r)?,
            )),
            BrainDispatch::Dreaming(r) => {
                Ok(BrainDispatchResponse::Dreaming(service.dispatch_dream(r)?))
            }
            BrainDispatch::Event(r) => Ok(BrainDispatchResponse::Event(service.dispatch_event(r)?)),
            BrainDispatch::Experience(r) => Ok(BrainDispatchResponse::Experience(
                service.dispatch_experience(r)?,
            )),
            BrainDispatch::Introspecting(r) => Ok(BrainDispatchResponse::Introspecting(
                service.dispatch_introspect(r)?,
            )),
            BrainDispatch::Level(r) => Ok(BrainDispatchResponse::Level(service.dispatch_level(r)?)),
            BrainDispatch::Lifecycle(r) => Ok(BrainDispatchResponse::Lifecycle(
                service.dispatch_lifecycle(r)?,
            )),
            BrainDispatch::Memory(r) => {
                Ok(BrainDispatchResponse::Memory(service.dispatch_memory(r)?))
            }
            BrainDispatch::Nature(r) => {
                Ok(BrainDispatchResponse::Nature(service.dispatch_nature(r)?))
            }
            BrainDispatch::Persona(r) => {
                Ok(BrainDispatchResponse::Persona(service.dispatch_persona(r)?))
            }
            BrainDispatch::Reflecting(r) => Ok(BrainDispatchResponse::Reflecting(
                service.dispatch_reflect(r)?,
            )),
            BrainDispatch::Search(r) => {
                Ok(BrainDispatchResponse::Search(service.dispatch_search(r)?))
            }
            BrainDispatch::Sensation(r) => Ok(BrainDispatchResponse::Sensation(
                service.dispatch_sensation(r)?,
            )),
            BrainDispatch::Sense(r) => Ok(BrainDispatchResponse::Sense(service.dispatch_sense(r)?)),
            BrainDispatch::Storage(r) => {
                Ok(BrainDispatchResponse::Storage(service.dispatch_storage(r)?))
            }
            BrainDispatch::Texture(r) => {
                Ok(BrainDispatchResponse::Texture(service.dispatch_texture(r)?))
            }
            BrainDispatch::Urge(r) => Ok(BrainDispatchResponse::Urge(service.dispatch_urge(r)?)),
        }
    }
}

// ── Unified dispatch ─────────────────────────────────────────────────

/// Top-level service dispatcher that routes protocol requests to the
/// appropriate scoped service.
///
/// Variants express the capability level: `Service` handles system-scoped
/// requests, `Brain` handles everything. Owns its state so it can live
/// as long as a connection requires — equally usable per-request
/// (construct, dispatch, drop) or per-connection (hold as a field).
#[derive(Clone)]
pub enum OneirosService {
    /// System-scoped only — can dispatch Actor, Brain, Tenant, Ticket.
    Service(Arc<ServiceState>),
    /// Full brain context — can dispatch all request types.
    Brain(BrainState),
}

impl OneirosService {
    /// Dispatch any protocol request to the appropriate service.
    pub fn dispatch(&self, request: impl Into<Requests>) -> Result<Responses, Error> {
        match request.into() {
            // System-scoped
            Requests::Actor(r) => Ok(self.state().system_service()?.dispatch(r)?.into()),
            Requests::Brain(r) => Ok(self.state().system_service()?.dispatch(r)?.into()),
            Requests::Tenant(r) => Ok(self.state().system_service()?.dispatch(r)?.into()),
            Requests::Ticket(r) => Ok(self.state().system_service()?.dispatch(r)?.into()),
            // Brain-scoped
            Requests::Agent(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Cognition(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Connection(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Dreaming(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Event(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Experience(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Introspecting(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Level(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Lifecycle(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Memory(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Nature(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Persona(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Reflecting(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Search(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Sensation(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Sense(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Storage(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Texture(r) => Ok(self.brain()?.dispatch(r)?.into()),
            Requests::Urge(r) => Ok(self.brain()?.dispatch(r)?.into()),
        }
    }

    fn state(&self) -> &ServiceState {
        match self {
            Self::Service(state) => state,
            Self::Brain(brain) => &brain.state,
        }
    }

    fn brain(&self) -> Result<&BrainState, Error> {
        match self {
            Self::Service(_) => Err(crate::BadRequests::NotHandled(
                "brain-scoped operations require brain context",
            )
            .into()),
            Self::Brain(brain) => Ok(brain),
        }
    }
}
