use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_service::*;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::Error;

#[derive(Debug, thiserror::Error)]
pub enum ActorContextError {
    #[error("Missing authorization header")]
    NoAuthHeader,
    #[error("Invalid auth header")]
    InvalidAuthHeader,
    #[error("Invalid or expired ticket")]
    InvalidOrExpiredTicket,
    #[error("Malformed token: {0}")]
    MalformedToken(#[from] TokenError),
}

pub struct ActorContext {
    db: Database,
    state: Arc<ServiceState>,
    event_tx: broadcast::Sender<Event>,
    source: Source,
}

/// Pressures below this threshold are excluded from brain-wide meta.
const PRESSURE_THRESHOLD: f64 = 0.80;

impl ActorContext {
    /// Create a scoped service for brain-level domain operations.
    pub(crate) fn service(&self) -> BrainService<'_> {
        BrainService::new(&self.db, &self.event_tx, self.source)
    }

    /// Wrap a pre-computed response with brain-wide pressure meta.
    ///
    /// Use this for handlers that bypass dispatch (e.g. binary storage uploads).
    pub(crate) fn respond(&self, data: impl Into<Responses>) -> Response {
        let meta = self.assemble_meta(None);
        Response {
            data: data.into(),
            meta,
        }
    }

    /// Dispatch a brain-scoped request and return a Response with pressure meta.
    pub(crate) fn dispatch(&self, request: impl Into<Requests>) -> Result<Response, crate::Error> {
        let request = request.into();
        let agent_scope = request.agent_scope().cloned();
        let data = self.dispatch_data(request)?;
        let meta = self.assemble_meta(agent_scope.as_ref());
        Ok(Response { data, meta })
    }

    fn dispatch_data(&self, request: Requests) -> Result<Responses, crate::Error> {
        let service = self.service();
        Ok(match request {
            Requests::Agent(r) => Responses::Agent(service.dispatch_agent(r)?),
            Requests::Cognition(r) => Responses::Cognition(service.dispatch_cognition(r)?),
            Requests::Connection(r) => Responses::Connection(service.dispatch_connection(r)?),
            Requests::Dreaming(r) => Responses::Dreaming(service.dispatch_dream(r)?),
            Requests::Event(r) => Responses::Event(service.dispatch_event(r)?),
            Requests::Experience(r) => Responses::Experience(service.dispatch_experience(r)?),
            Requests::Introspecting(r) => Responses::Introspecting(service.dispatch_introspect(r)?),
            Requests::Level(r) => Responses::Level(service.dispatch_level(r)?),
            Requests::Lifecycle(r) => Responses::Lifecycle(service.dispatch_lifecycle(r)?),
            Requests::Memory(r) => Responses::Memory(service.dispatch_memory(r)?),
            Requests::Nature(r) => Responses::Nature(service.dispatch_nature(r)?),
            Requests::Persona(r) => Responses::Persona(service.dispatch_persona(r)?),
            Requests::Pressure(r) => Responses::Pressure(service.dispatch_pressure(r)?),
            Requests::Reflecting(r) => Responses::Reflecting(service.dispatch_reflect(r)?),
            Requests::Search(r) => Responses::Search(service.dispatch_search(r)?),
            Requests::Sensation(r) => Responses::Sensation(service.dispatch_sensation(r)?),
            Requests::Sense(r) => Responses::Sense(service.dispatch_sense(r)?),
            Requests::Storage(r) => Responses::Storage(service.dispatch_storage(r)?),
            Requests::Texture(r) => Responses::Texture(service.dispatch_texture(r)?),
            Requests::Urge(r) => Responses::Urge(service.dispatch_urge(r)?),
            // System-scoped requests are not handled by ActorContext
            _ => {
                return Err(oneiros_service::BadRequests::NotHandled(
                    "system-scoped operations are not available on brain endpoints",
                )
                .into());
            }
        })
    }

    fn assemble_meta(&self, agent_scope: Option<&AgentName>) -> Option<ResponseMeta> {
        let readings = self.pressure_readings(agent_scope).ok()?;
        if readings.is_empty() {
            None
        } else {
            Some(ResponseMeta { pressure: readings })
        }
    }

    fn pressure_readings(
        &self,
        agent_scope: Option<&AgentName>,
    ) -> Result<Vec<PressureReading>, oneiros_service::Error> {
        let pressures = match agent_scope {
            Some(name) => match self.db.get_agent(name)? {
                Some(a) => self.db.list_pressures_for_agent(&a.id.to_string())?,
                None => vec![],
            },
            None => {
                let agents = self.db.list_agents()?;
                let mut all = Vec::new();
                for a in agents {
                    let mut ap = self.db.list_pressures_for_agent(&a.id.to_string())?;
                    all.append(&mut ap);
                }
                all.into_iter()
                    .filter(|p| p.urgency() >= PRESSURE_THRESHOLD)
                    .collect()
            }
        };
        let urges = self.db.list_urges()?;
        Ok(PressureReading::from_pressures_and_urges(pressures, &urges))
    }

    /// Construct a [`BrainState`] from the database, consuming the context.
    pub(crate) fn into_oneiros_state(self) -> OneirosService {
        OneirosService::Brain(BrainState::new(self.state, self.db))
    }
}

impl FromRequestParts<Arc<ServiceState>> for ActorContext {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ServiceState>,
    ) -> Result<Self, Self::Rejection> {
        let token_string = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(ActorContextError::NoAuthHeader)?
            .strip_prefix("Bearer ")
            .ok_or(ActorContextError::InvalidAuthHeader)?;

        let token = Token(token_string.to_owned());
        let claims = token.decode().map_err(ActorContextError::from)?;

        let (brain_path, event_tx) = {
            let db = state.lock_database()?;

            if !db.validate_ticket(token.as_str())? {
                Err(ActorContextError::InvalidOrExpiredTicket)?;
            }

            let path = db
                .get_brain_path(claims.tenant_id.to_string(), claims.brain_id.to_string())?
                .ok_or(NotFound::Brain(claims.brain_id))?;

            (path, state.event_sender().clone())
        };

        let brain_db = Database::open_brain(brain_path)?;

        let source = Source {
            actor_id: claims.actor_id,
            tenant_id: claims.tenant_id,
        };

        Ok(ActorContext {
            db: brain_db,
            state: state.clone(),
            event_tx,
            source,
        })
    }
}
