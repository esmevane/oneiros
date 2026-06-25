use aide::axum::{ApiRouter, routing};
use axum::Json;

use crate::*;

pub(crate) struct ActorRouter;

impl ActorRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/actors",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(ListActors::handler, |op| {
                        resource_op!(op, ActorDocs::List).response::<200, Json<ActorsResponse>>()
                    })
                    .post_with(CreateActor::handler, |op| {
                        resource_op!(op, ActorDocs::Create)
                            .response::<201, Json<ActorCreatedResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(GetActor::handler, |op| {
                        resource_op!(op, ActorDocs::Get)
                            .input::<IdPathParam<ActorId>>()
                            .response::<200, Json<ActorFoundResponse>>()
                    }),
                ),
        )
    }
}
