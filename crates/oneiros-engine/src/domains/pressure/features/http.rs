use aide::axum::{ApiRouter, routing};
use axum::{Json, extract::Path};

use crate::*;

pub struct PressureRouter;

impl PressureRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/pressures",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, PressureDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{agent}",
                    routing::get_with(get, |op| {
                        resource_op!(op, PressureDocs::Get).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn get(
    context: ProjectContext,
    Path(agent): Path<AgentName>,
) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(
        PressureService::get(
            &context,
            &GetPressure::builder_v1().agent(agent).build().into(),
        )
        .await?,
    ))
}

async fn list(context: ProjectContext) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(PressureService::list(&context).await?))
}
