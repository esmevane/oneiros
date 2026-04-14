use std::sync::{Arc, Mutex, OnceLock};

use axum::{extract::FromRequestParts, http::request::Parts};
use tokio::sync::broadcast;

use crate::*;

/// Shared state for the HTTP server.
///
/// Carries the system context (always available), a shared broadcast
/// channel for SSE subscribers, and resolves brain context per-request
/// via Bearer token.
#[derive(Clone)]
pub(crate) struct ServerState {
    config: Config,
    broadcast: broadcast::Sender<StoredEvent>,
    canons: CanonIndex,
    bridge: Bridge,
    bus: Arc<OnceLock<(EventBus, ProjectorHandle)>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ServerStateError {
    #[error("failed to read or generate host secret key: {0}")]
    HostKey(#[from] std::io::Error),
    #[error(transparent)]
    Bridge(#[from] BridgeError),
}

impl ServerState {
    /// Construct a server state with a bound iroh bridge. Loads (or
    /// generates) the host secret key from disk and binds a `Bridge`.
    pub(crate) async fn bind(config: Config) -> Result<Self, ServerStateError> {
        let secret = config.ensure_host_secret_key()?;
        let bridge = Bridge::bind(secret).await?;

        let (broadcast, _) = broadcast::channel(256);
        let canons = CanonIndex::new();

        Ok(Self {
            config,
            broadcast,
            canons,
            bridge,
            bus: Arc::new(OnceLock::new()),
        })
    }

    /// Get or create the shared event bus and projector.
    fn bus_and_projector(&self, config: &Config) -> Result<(EventBus, ProjectorHandle), EventError> {
        let pair = self.bus.get_or_init(|| {
            let db = config.brain_db().expect("brain db");
            EventLog::new(&db).migrate().expect("event log migration");

            let projections = Projections::project();
            projections.migrate(&db).expect("projection migration");

            let db = Arc::new(Mutex::new(db));
            let bus = EventBus::new(db.clone());

            let handle = Projector::spawn_brain(
                db,
                projections,
                self.canons.clone(),
                config.brain.clone(),
                bus.broadcast(),
            );

            (bus, handle)
        });

        Ok(pair.clone())
    }


    /// The bound bridge.
    pub(crate) fn bridge(&self) -> &Bridge {
        &self.bridge
    }

    /// The host's identity (key + address).
    pub(crate) fn host_identity(&self) -> HostIdentity {
        self.bridge.host_identity()
    }

    /// The canon index — shared CRDT state for all brains.
    pub(crate) fn canons(&self) -> &CanonIndex {
        &self.canons
    }

    /// Hydrate all canons from event logs. Best-effort — skips
    /// databases that don't exist yet (pre-init).
    pub(crate) fn hydrate(&self) {
        // System canon
        let _ = self.canons.hydrate_system(&self.config);

        // Brain canon for the configured brain
        let _ = self.canons.hydrate_brain(&self.config, &self.config.brain);
    }

    /// The server configuration.
    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    /// The token for the configured brain, if one exists.
    pub(crate) fn token(&self) -> Option<Token> {
        self.config.token()
    }

    /// The brain name from the server config.
    pub(crate) fn brain_name(&self) -> &BrainName {
        &self.config.brain
    }

    /// The shared broadcast sender for SSE event streaming.
    pub(crate) fn broadcast(&self) -> &broadcast::Sender<StoredEvent> {
        &self.broadcast
    }

    /// The projector handle for the configured brain.
    pub(crate) fn projector(&self, config: &Config) -> Result<ProjectorHandle, EventError> {
        let (_, handle) = self.bus_and_projector(config)?;
        Ok(handle)
    }

    /// Build a project context with the shared bus.
    pub(crate) fn project_context(&self, config: Config) -> Result<ProjectContext, EventError> {
        let (bus, _) = self.bus_and_projector(&config)?;
        Ok(ProjectContext::with_bus(config, bus))
    }

    /// Build a system context with shared canon.
    pub(crate) fn system_context(&self) -> SystemContext {
        let canon = self.canons.system().clone();
        SystemContext::with_canon(self.config.clone(), canon)
    }
}

impl FromRequestParts<ServerState> for SystemContext {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.system_context())
    }
}

impl FromRequestParts<ServerState> for ProjectContext {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        // Extract Bearer token
        let token_str = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::NoAuthHeader)?
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidAuthHeader)?;

        // Decode claims from the self-describing token
        let token = Token::from(token_str)
            .decode()
            .map_err(|_| AuthError::InvalidToken)?;

        // Revocation check — verify the ticket still exists in the DB
        let system = state.config.system();
        let ticket = TicketRepo::new(&system)
            .get_by_token(token_str)
            .await
            .map_err(|_| AuthError::InvalidToken)?
            .ok_or(AuthError::InvalidToken)?;

        match (
            ticket.actor_id == token.actor_id,
            ticket.brain_id == token.brain_id,
            true, // ticket.tenant_id == token.tenant_id,
        ) {
            (true, true, true) => {}
            _ => return Err(AuthError::InvalidToken),
        }

        // Assemble ProjectContext with shared bus
        let mut config = state.config.clone();
        config.brain = ticket.brain_name;

        state
            .project_context(config)
            .map_err(|_| AuthError::InvalidToken)
    }
}
