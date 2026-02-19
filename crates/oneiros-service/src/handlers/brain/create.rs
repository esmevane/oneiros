use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_protocol::*;
use std::sync::Arc;

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum CreateBrainError {
    #[error("Malformed input: {0}")]
    MalformedId(#[from] IdParseError),
}

pub(crate) async fn handler(
    State(state): State<Arc<ServiceState>>,
    Json(request): Json<CreateBrainRequest>,
) -> Result<(StatusCode, Json<BrainInfo>), Error> {
    let db = state.database.lock().map_err(|_| Error::DatabasePoisoned)?;

    let tenant_id: TenantId = db
        .get_tenant_id()?
        .ok_or(PreconditionFailure::NoTenant)?
        .parse()
        .map_err(CreateBrainError::from)
        .map_err(BadRequests::from)?;

    let actor_id: ActorId = db
        .get_actor_id(tenant_id.to_string())?
        .ok_or(PreconditionFailure::NoActor)?
        .parse()
        .map_err(CreateBrainError::from)
        .map_err(BadRequests::from)?;

    if db.brain_exists(tenant_id.to_string(), request.name.as_str())? {
        Err(Conflicts::Brain(request.name.clone()))?;
    }

    let brains_dir = state.data_dir.join("brains");
    oneiros_fs::FileOps.ensure_dir(&brains_dir)?;

    let path = brains_dir.join(format!("{}.db", request.name));

    Database::create_brain_db(&path)?;

    let brain_id = BrainId::new();

    let entity = Identity::new(
        brain_id,
        Brain {
            tenant_id,
            path,
            name: request.name,
            status: BrainStatus::Active,
        },
    );

    let event = Events::Brain(BrainEvents::BrainCreated(entity.clone()));

    db.log_event(&event, projections::SYSTEM_PROJECTIONS)?;

    let token = Token::issue(TokenClaims {
        brain_id,
        tenant_id,
        actor_id,
    });

    let ticket_event = Events::Ticket(TicketEvents::TicketIssued(Identity::new(
        TicketId::new(),
        Ticket {
            token: token.clone(),
            created_by: actor_id,
        },
    )));

    db.log_event(&ticket_event, projections::SYSTEM_PROJECTIONS)?;

    let info = BrainInfo { entity, token };

    Ok((StatusCode::CREATED, Json(info)))
}
