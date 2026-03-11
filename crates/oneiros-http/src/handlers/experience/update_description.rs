use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(mut request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    request.id = id;

    Ok((
        StatusCode::OK,
        Json(ticket.dispatch(ExperienceRequests::UpdateExperienceDescription(request))?),
    ))
}
