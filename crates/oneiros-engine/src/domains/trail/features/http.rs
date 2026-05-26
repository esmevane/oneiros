use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::Path};

use crate::*;

pub(crate) struct TrailRouter;

impl TrailRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/trail",
            ApiRouter::<ServerState>::new()
                .api_route(
                    "/of/{ref}",
                    routing::get_with(of, |op| {
                        resource_op!(op, TrailDocs::Of).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/from/{event_id}",
                    routing::get_with(from, |op| {
                        resource_op!(op, TrailDocs::From).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn of(
    scope: Scope<AtBookmark>,
    Path(entity_ref): Path<RefToken>,
) -> Result<Json<TrailResponse>, TrailError> {
    Ok(Json(TrailService::of(&scope, &entity_ref).await?))
}

async fn from(
    scope: Scope<AtBookmark>,
    Path(event_id): Path<EventId>,
) -> Result<Json<TrailResponse>, TrailError> {
    Ok(Json(TrailService::from(&scope, event_id).await?))
}
