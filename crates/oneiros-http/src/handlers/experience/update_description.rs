use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(mut request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<ExperienceResponses>), Error> {
    request.id = id;

    let response = ticket
        .service()
        .dispatch_experience(ExperienceRequests::UpdateExperienceDescription(request))?;

    Ok((StatusCode::OK, Json(response)))
}
