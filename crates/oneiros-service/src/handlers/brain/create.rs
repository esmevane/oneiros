use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use oneiros_db::Database;
use oneiros_model::*;
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

    if db
        .brain_exists(tenant_id.to_string(), request.name.as_str())
        .is_ok()
    {
        Err(Conflicts::Brain(request.name.clone()))?;
    }

    let brains_dir = state.data_dir.join("brains");
    oneiros_fs::FileOps.ensure_dir(&brains_dir)?;

    let path = brains_dir.join(format!("{}.db", request.name));

    Database::create_brain_db(&path)?;

    let brain = HasPath::new(
        path,
        Brain {
            tenant_id,
            name: request.name,
            status: BrainStatus::Active,
        },
    );

    let brain_id = BrainId::new();
    let brain = Identity::new(brain_id, brain);

    let event = Events::Brain(BrainEvents::BrainCreated(brain));

    db.log_event(&event, projections::system::ALL)?;

    let token = Token::issue(TokenClaims {
        brain_id,
        tenant_id,
        actor_id,
    });

    let ticket = Identity::new(
        TicketId::new(),
        Ticket {
            token: token.clone(),
            created_by: actor_id,
        },
    );

    let ticket_event = Events::Ticket(TicketEvents::TicketIssued(ticket));

    db.log_event(&ticket_event, projections::system::ALL)?;

    let info = BrainInfo {
        entity: brain_id,
        token,
    };

    Ok((StatusCode::CREATED, Json(info)))
}
