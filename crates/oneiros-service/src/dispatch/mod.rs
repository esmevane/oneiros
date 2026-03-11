mod brain;
mod system;
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

    // ── Shared helpers ───────────────────────────────────────────────

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

    /// Persist an observational marker event (no projections) then broadcast.
    pub(crate) fn log_marker(&self, db: &Database, event: &Events) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source());
        let persisted = db.log_event(&new_event, &[])?;
        let _ = self.state().event_sender().send(persisted.clone());
        Ok(persisted)
    }

    // ── Top-level dispatch ───────────────────────────────────────────

    /// Dispatch any protocol request and assemble a Response with pressure meta.
    pub fn dispatch(&self, request: impl Into<Requests>) -> Result<Response, Error> {
        let request = request.into();
        let agent_scope = request.agent_scope().cloned();
        let data = self.dispatch_data(request)?;
        let meta = self.assemble_meta(agent_scope.as_ref());
        Ok(Response { data, meta })
    }

    /// Dispatch without assembling meta — returns bare Responses.
    fn dispatch_data(&self, request: Requests) -> Result<Responses, Error> {
        match request {
            // System-scoped domains
            Requests::Actor(r) => Ok(Responses::Actor(self.dispatch_actor(r)?)),
            Requests::Brain(r) => Ok(Responses::Brain(self.dispatch_brain(r)?)),
            Requests::Tenant(r) => Ok(Responses::Tenant(self.dispatch_tenant(r)?)),
            Requests::Ticket(r) => Ok(Responses::Ticket(self.dispatch_ticket(r)?)),
            // Brain-scoped domains
            Requests::Agent(r) => Ok(Responses::Agent(self.dispatch_agent(r)?)),
            Requests::Cognition(r) => Ok(Responses::Cognition(self.dispatch_cognition(r)?)),
            Requests::Connection(r) => Ok(Responses::Connection(self.dispatch_connection(r)?)),
            Requests::Dreaming(r) => Ok(Responses::Dreaming(self.dispatch_dream(r)?)),
            Requests::Event(r) => Ok(Responses::Event(self.dispatch_event(r)?)),
            Requests::Experience(r) => Ok(Responses::Experience(self.dispatch_experience(r)?)),
            Requests::Introspecting(r) => {
                Ok(Responses::Introspecting(self.dispatch_introspect(r)?))
            }
            Requests::Level(r) => Ok(Responses::Level(self.dispatch_level(r)?)),
            Requests::Lifecycle(r) => Ok(Responses::Lifecycle(self.dispatch_lifecycle(r)?)),
            Requests::Memory(r) => Ok(Responses::Memory(self.dispatch_memory(r)?)),
            Requests::Nature(r) => Ok(Responses::Nature(self.dispatch_nature(r)?)),
            Requests::Persona(r) => Ok(Responses::Persona(self.dispatch_persona(r)?)),
            Requests::Pressure(r) => Ok(Responses::Pressure(self.dispatch_pressure(r)?)),
            Requests::Reflecting(r) => Ok(Responses::Reflecting(self.dispatch_reflect(r)?)),
            Requests::Search(r) => Ok(Responses::Search(self.dispatch_search(r)?)),
            Requests::Sensation(r) => Ok(Responses::Sensation(self.dispatch_sensation(r)?)),
            Requests::Sense(r) => Ok(Responses::Sense(self.dispatch_sense(r)?)),
            Requests::Storage(r) => Ok(Responses::Storage(self.dispatch_storage(r)?)),
            Requests::Texture(r) => Ok(Responses::Texture(self.dispatch_texture(r)?)),
            Requests::Urge(r) => Ok(Responses::Urge(self.dispatch_urge(r)?)),
        }
    }

    // ── Pressure meta ────────────────────────────────────────────────

    /// Build pressure readings, scoped to an agent or brain-wide above threshold.
    ///
    /// Agent-scoped: all pressures for that agent.
    /// Brain-wide: only pressures above PRESSURE_THRESHOLD.
    pub fn pressure_readings(
        &self,
        agent_scope: Option<&AgentName>,
    ) -> Result<Vec<PressureReading>, Error> {
        let db = self.lock_brain()?;
        self.pressure_readings_from(&db, agent_scope)
    }

    /// Inner implementation — takes a pre-locked `&Database` to avoid
    /// double-locking when called from a method that already holds the guard.
    pub(crate) fn pressure_readings_from(
        &self,
        db: &Database,
        agent_scope: Option<&AgentName>,
    ) -> Result<Vec<PressureReading>, Error> {
        let pressures = match agent_scope {
            Some(name) => match db.get_agent(name)? {
                Some(a) => db.list_pressures_for_agent(&a.id.to_string())?,
                None => vec![],
            },
            None => {
                let agents = db.list_agents()?;
                let mut all = Vec::new();
                for a in agents {
                    let mut ap = db.list_pressures_for_agent(&a.id.to_string())?;
                    all.append(&mut ap);
                }
                all.into_iter()
                    .filter(|p| p.urgency() >= PRESSURE_THRESHOLD)
                    .collect()
            }
        };

        let urges = db.list_urges()?;
        Ok(PressureReading::from_pressures_and_urges(pressures, &urges))
    }

    /// Assemble pressure meta for a response.
    ///
    /// Agent-scoped: all pressures for that agent.
    /// Brain-scoped: only pressures above threshold.
    /// System-scoped (no brain): None.
    pub fn assemble_meta(&self, agent_scope: Option<&AgentName>) -> Option<ResponseMeta> {
        let readings = self.pressure_readings(agent_scope).ok()?;

        if readings.is_empty() {
            None
        } else {
            Some(ResponseMeta { pressure: readings })
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
        let readings = self.pressure_readings_from(db, agent_scope).ok()?;

        if readings.is_empty() {
            None
        } else {
            Some(ResponseMeta { pressure: readings })
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
