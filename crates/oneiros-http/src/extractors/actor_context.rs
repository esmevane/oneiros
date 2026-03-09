use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use oneiros_db::Database;
use oneiros_model::{Event, NotFound, Source, Token, TokenError};
use oneiros_service::{BrainService, BrainState, OneirosService, ServiceState};
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

impl ActorContext {
    /// Create a scoped service for brain-level domain operations.
    pub(crate) fn service(&self) -> BrainService<'_> {
        BrainService::new(&self.db, &self.event_tx, self.source)
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
