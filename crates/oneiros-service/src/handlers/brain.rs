use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use oneiros_client::{BrainInfo, CreateBrainRequest};
use oneiros_db::Database;
use oneiros_model::{Brain, BrainEvents, BrainStatus, Events, Id, projections};

use crate::Error;
use crate::state::ServiceState;

pub(crate) async fn create_brain(
    State(state): State<Arc<ServiceState>>,
    Json(request): Json<CreateBrainRequest>,
) -> Result<(StatusCode, Json<BrainInfo>), Error> {
    let db = state.database.lock().map_err(|_| Error::LockPoisoned)?;

    let tenant_id = db.get_tenant_id()?.ok_or_else(|| {
        Error::NotInitialized("No tenant found. Run `oneiros system init` first.".into())
    })?;

    if db.brain_exists(&tenant_id, request.name.as_str())? {
        return Err(Error::Conflict(format!(
            "Brain '{}' already exists.",
            request.name,
        )));
    }

    let brains_dir = state.data_dir.join("brains");
    oneiros_fs::FileOps.ensure_dir(&brains_dir)?;

    let brain_path = brains_dir.join(format!("{}.db", request.name));

    Database::create_brain_db(&brain_path)?;

    let brain_id = Id::new();

    let event = Events::Brain(BrainEvents::BrainCreated(Brain {
        brain_id,
        tenant_id: Id(uuid::Uuid::parse_str(&tenant_id).unwrap_or_else(|_| uuid::Uuid::now_v7())),
        name: request.name.clone(),
        path: brain_path.clone(),
    }));

    db.log_event(&event, projections::SYSTEM_PROJECTIONS)?;

    let info = BrainInfo {
        id: brain_id,
        name: request.name,
        path: brain_path,
        status: BrainStatus::Active,
    };

    Ok((StatusCode::CREATED, Json(info)))
}
