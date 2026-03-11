use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use oneiros_model::*;
use oneiros_service::OneirosService;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new().route("/", post(create))
}

async fn create(
    State(state): State<OneirosService>,
    Json(request): Json<CreateBrainRequest>,
) -> Result<(StatusCode, Json<BrainResponses>), Error> {
    let response = state.dispatch(BrainRequests::CreateBrain(request))?;

    let Responses::Brain(brain_response) = response.data else {
        Err(Error::ProjectExtractionFailure)?
    };

    Ok((StatusCode::CREATED, Json(brain_response)))
}
