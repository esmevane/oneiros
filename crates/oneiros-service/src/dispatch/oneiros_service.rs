use oneiros_db::{Database, Projection};
use oneiros_model::*;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::*;

/// Pressures below this threshold are excluded from brain-wide meta.
/// Agent-scoped pressures are always included regardless of threshold.
pub(crate) const PRESSURE_THRESHOLD: f64 = 0.80;

/// Top-level service dispatcher that routes protocol requests to the
/// appropriate scoped domain methods.
///
/// `System` handles system-scoped requests (Actor, Brain, Tenant, Ticket).
/// `Brain` handles all requests — system and brain-scoped alike.
///
/// Owns its state so it can live as long as a connection requires —
/// equally usable per-request (construct, dispatch, drop) or
/// per-connection (hold as a field).
#[derive(Clone)]
pub enum OneirosService {
    /// System-scoped only — can dispatch Actor, Brain, Tenant, Ticket.
    System {
        state: Arc<ServiceState>,
        source: Source,
    },
    /// Full brain context — can dispatch all request types.
    Brain {
        state: Arc<ServiceState>,
        database: Arc<Mutex<Database>>,
        source: Source,
    },
}

impl OneirosService {
    /// Create a system-scoped service using the state's default source identity.
    pub fn system(state: Arc<ServiceState>) -> Self {
        let source = state.source();
        Self::System { state, source }
    }

    /// Create a brain-scoped service using the state's default source identity.
    pub fn brain(state: Arc<ServiceState>, database: Database) -> Self {
        let source = state.source();
        Self::Brain {
            state,
            database: Arc::new(Mutex::new(database)),
            source,
        }
    }

    pub fn upgrade(&self, token: impl Into<Token>) -> Result<Self, Error> {
        let token = token.into();
        let TokenClaims {
            brain_id,
            actor_id,
            tenant_id,
        } = token.decode()?;

        let brain_path = {
            let db = self.state().lock_database()?;

            if !db.validate_ticket(token.as_str())? {
                Err(Error::InvalidOrExpiredTicket)?;
            }

            db.get_brain_path(tenant_id.to_string(), brain_id.to_string())?
                .ok_or(NotFound::Brain(brain_id))?
        };

        let brain_db = Database::open_brain(brain_path)?;

        let source = Source {
            actor_id,
            tenant_id,
        };

        let state = match self {
            Self::System { state, .. } => state.clone(),
            Self::Brain { state, .. } => state.clone(),
        };

        Ok(Self::Brain {
            state,
            database: Arc::new(Mutex::new(brain_db)),
            source,
        })
    }

    /// Access the shared service state regardless of variant.
    pub fn state(&self) -> &ServiceState {
        match self {
            Self::System { state, .. } => state,
            Self::Brain { state, .. } => state,
        }
    }

    /// Access the source identity.
    pub fn source(&self) -> Source {
        match self {
            Self::System { source, .. } => *source,
            Self::Brain { source, .. } => *source,
        }
    }

    /// Acquire the brain database lock.
    ///
    /// Returns `Error::NoBrainContext` if this is a system-scoped service.
    pub fn lock_brain(&self) -> Result<MutexGuard<'_, Database>, Error> {
        match self {
            Self::System { .. } => Err(Error::NoBrainContext),
            Self::Brain { database, .. } => database.lock().map_err(|_| Error::DatabasePoisoned),
        }
    }

    /// Acquire the system database lock.
    pub fn lock_system(&self) -> Result<MutexGuard<'_, Database>, Error> {
        self.state().lock_database()
    }

    /// Construct an event sink for the current service context.
    ///
    /// Takes the database and projection set — the sink knows where
    /// to write and what projections to run. Dispatchers just emit events.
    pub fn effects<'a>(
        &'a self,
        db: &'a Database,
        projections: &'a [&'a [Projection]],
    ) -> ServiceEffects<'a> {
        ServiceEffects::builder()
            .db(db)
            .projections(projections)
            .source(self.source())
            .sender(self.state().event_sender())
            .build()
    }

    /// Create a BrainScope — a locked brain context that resources pull from.
    ///
    /// The scope owns the lock and the ingredients for effects construction.
    /// Resources construct themselves from the scope and dispatch through it.
    pub fn brain_scope<'a>(&'a self) -> Result<BrainScope<'a>, Error> {
        let db = self.lock_brain()?;

        Ok(BrainScope::new(
            db,
            self.source(),
            self.state().event_sender(),
            projections::BRAIN,
        ))
    }

    /// Create a SystemScope — a locked system context for system-scoped stores.
    pub fn system_scope(&self) -> Result<SystemScope<'_>, Error> {
        let db = self.lock_system()?;

        Ok(SystemScope::new(
            db,
            self.source(),
            self.state().event_sender(),
            projections::SYSTEM,
        ))
    }

    /// Persist a state-changing event (runs the given projections) then broadcast.
    pub(crate) fn log_and_broadcast(
        &self,
        db: &Database,
        event: &Events,
        projections: &[&[Projection]],
    ) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source());
        let persisted = db.log_event(&new_event, projections)?;

        let _ = self.state().event_sender().send(persisted.clone());

        Ok(persisted)
    }

    // ── Top-level dispatch ───────────────────────────────────────────

    /// Dispatch any protocol request and assemble a Response with pressure meta.
    pub fn dispatch(&self, request: impl Into<Requests>) -> Result<Response, Error> {
        let request = request.into();
        let agent_scope = request.agent_scope().cloned();
        let data = self.dispatch_data(request)?;
        let ref_token = extract_ref_token(&data);
        let meta = self.assemble_meta_with_ref_token(agent_scope.as_ref(), ref_token);
        Ok(Response { data, meta })
    }

    /// Dispatch without assembling meta — returns bare Responses.
    fn dispatch_data(&self, request: Requests) -> Result<Responses, Error> {
        Ok(match request {
            // System-scoped domains
            Requests::Actor(r) => ActorStore
                .dispatch(RequestContext::new(r, &self.system_scope()?))?
                .into(),
            Requests::Brain(r) => BrainStore {
                data_dir: self.state().data_dir().to_path_buf(),
            }
            .dispatch(RequestContext::new(r, &self.system_scope()?))?
            .into(),
            Requests::Tenant(r) => TenantStore
                .dispatch(RequestContext::new(r, &self.system_scope()?))?
                .into(),
            Requests::Ticket(r) => TicketStore
                .dispatch(RequestContext::new(r, &self.system_scope()?))?
                .into(),

            // Brain-scoped domains — store-dispatched
            Requests::Agent(r) => AgentStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Cognition(r) => CognitionStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Connection(r) => ConnectionStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Experience(r) => ExperienceStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Level(r) => LevelStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Memory(r) => MemoryStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Nature(r) => NatureStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Persona(r) => PersonaStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Pressure(r) => PressureStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Search(r) => SearchStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Sensation(r) => SensationStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Storage(r) => StorageStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Texture(r) => TextureStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Urge(r) => UrgeStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),

            Requests::Dreaming(r) => DreamStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Event(r) => EventStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Introspecting(r) => IntrospectStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Lifecycle(r) => LifecycleStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Reflecting(r) => ReflectStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
            Requests::Sense(r) => SenseStore
                .dispatch(RequestContext::new(r, &self.brain_scope()?))?
                .into(),
        })
    }

    // ── Pressure meta ────────────────────────────────────────────────

    /// Build compact pressure summaries, scoped to an agent or brain-wide above threshold.
    ///
    /// Agent-scoped: all pressures for that agent.
    /// Brain-wide: only pressures above PRESSURE_THRESHOLD.
    pub fn pressure_summaries(
        &self,
        agent_scope: Option<&AgentName>,
    ) -> Result<Vec<PressureSummary>, Error> {
        let db = self.lock_brain()?;
        self.pressure_summaries_from(&db, agent_scope)
    }

    /// Inner implementation — takes a pre-locked `&Database` to avoid
    /// double-locking when called from a method that already holds the guard.
    pub(crate) fn pressure_summaries_from(
        &self,
        db: &Database,
        agent_scope: Option<&AgentName>,
    ) -> Result<Vec<PressureSummary>, Error> {
        let pressures = match agent_scope {
            Some(name) => match db.get_agent(name)? {
                Some(a) => db.list_pressures_for_agent(&a.id.to_string())?,
                None => vec![],
            },
            None => db
                .list_all_pressures()?
                .into_iter()
                .filter(|p| p.urgency() >= PRESSURE_THRESHOLD)
                .collect(),
        };

        Ok(pressures.iter().map(PressureSummary::from).collect())
    }

    /// Assemble pressure meta for a response.
    ///
    /// Agent-scoped: all pressures for that agent.
    /// Brain-scoped: only pressures above threshold.
    /// System-scoped (no brain): None.
    pub fn assemble_meta(&self, agent_scope: Option<&AgentName>) -> Option<ResponseMeta> {
        self.assemble_meta_with_ref_token(agent_scope, None)
    }

    /// Assemble meta with both pressure summaries and an optional ref token.
    ///
    /// Returns `None` only when there are no pressures and no ref_token to include.
    fn assemble_meta_with_ref_token(
        &self,
        agent_scope: Option<&AgentName>,
        ref_token: Option<RefToken>,
    ) -> Option<ResponseMeta> {
        let summaries = self
            .pressure_summaries(agent_scope)
            .ok()
            .unwrap_or_default();

        if summaries.is_empty() && ref_token.is_none() {
            None
        } else {
            Some(ResponseMeta {
                pressure: summaries,
                ref_token,
            })
        }
    }

    /// Assemble pressure meta using a pre-locked database.
    ///
    /// Used internally to avoid re-locking in contexts that already hold
    /// the brain mutex guard.
    pub(crate) fn assemble_meta_from(
        &self,
        db: &Database,
        agent_scope: Option<&AgentName>,
    ) -> Option<ResponseMeta> {
        let summaries = self
            .pressure_summaries_from(db, agent_scope)
            .ok()
            .unwrap_or_default();

        if summaries.is_empty() {
            None
        } else {
            Some(ResponseMeta {
                pressure: summaries,
                ..Default::default()
            })
        }
    }

    // ── Storage binary bypass ────────────────────────────────────────

    /// Set storage content. Bypasses the dispatch enum because
    /// SetStorageRequest carries binary data that doesn't fit the
    /// JSON protocol envelope.
    pub fn set_storage(&self, request: SetStorageRequest) -> Result<Response, Error> {
        // Acquire and release the lock in a tight scope before calling
        // assemble_meta, which would re-lock the same mutex and deadlock.
        let (data, meta) = {
            let db = self.lock_brain()?;
            let data = self.set_storage_from(&db, request)?;
            let meta = self.assemble_meta_from(&db, None);
            (data, meta)
        };
        Ok(Response {
            data: data.into(),
            meta,
        })
    }

    pub(crate) fn set_storage_from(
        &self,
        db: &Database,
        request: SetStorageRequest,
    ) -> Result<StorageResponses, Error> {
        let blob_content = BlobContent::create(&request.data)?;

        db.put_blob(&blob_content)?;

        let entry = StorageEntry::init(request.key, &request.description, blob_content.hash);
        let event = Events::Storage(StorageEvents::StorageSet(entry.clone()));

        self.log_and_broadcast(db, &event, projections::BRAIN)?;

        Ok(StorageResponses::StorageSet(entry))
    }

    /// Get raw storage content bytes. Bypasses the dispatch enum because
    /// it returns raw bytes rather than a protocol response.
    pub fn get_storage_content(&self, key: &StorageKey) -> Result<Vec<u8>, Error> {
        let db = self.lock_brain()?;
        self.get_storage_content_from(&db, key)
    }

    pub(crate) fn get_storage_content_from(
        &self,
        db: &Database,
        key: &StorageKey,
    ) -> Result<Vec<u8>, Error> {
        let entry = db.get_storage(key)?.ok_or(NotFound::Storage(key.clone()))?;

        let blob = db
            .get_blob(&entry.hash)?
            .ok_or(DataIntegrity::BlobMissing(entry.hash.clone()))?;

        let decompressed = blob.data.decompressed()?;

        Ok(decompressed)
    }
}

/// Extract a `RefToken` from entity-creating response variants.
///
/// Only responses that create a new entity carry a ref_token. All other
/// response variants return `None`.
fn extract_ref_token(data: &Responses) -> Option<RefToken> {
    match data {
        Responses::Cognition(CognitionResponses::CognitionAdded(c)) => Some(c.ref_token()),
        Responses::Memory(MemoryResponses::MemoryAdded(m)) => Some(m.ref_token()),
        Responses::Experience(ExperienceResponses::ExperienceCreated(e)) => Some(e.ref_token()),
        Responses::Connection(ConnectionResponses::ConnectionCreated(c)) => Some(c.ref_token()),
        Responses::Agent(AgentResponses::AgentCreated(a)) => Some(RefToken::new(Ref::agent(a.id))),
        _ => None,
    }
}
