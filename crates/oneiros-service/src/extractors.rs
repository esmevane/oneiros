use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use oneiros_db::Database;
use oneiros_model::{Token, TokenError};
use std::sync::Arc;

use crate::Error;
use crate::error::NotFound;
use crate::state::ServiceState;

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
    pub db: Database,
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

        let brain_path = {
            let db = state.database.lock().map_err(|_| Error::DatabasePoisoned)?;

            if !db.validate_ticket(token.as_str())? {
                Err(ActorContextError::InvalidOrExpiredTicket)?;
            }

            db.get_brain_path(&claims.tenant_id, &claims.brain_id)?
                .ok_or(NotFound::Brain(claims.brain_id))?
        };

        let brain_db = Database::open_brain(brain_path)?;

        Ok(ActorContext { db: brain_db })
    }
}
