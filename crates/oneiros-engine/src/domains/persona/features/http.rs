use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct PersonaRouter;

impl PersonaRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/personas",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, PersonaDocs::List).security_requirement("BearerToken")
                    }),
                )
                .api_route(
                    "/{name}",
                    routing::put_with(set, |op| {
                        resource_op!(op, PersonaDocs::Set)
                            .security_requirement("BearerToken")
                            .response::<200, Json<PersonaResponse>>()
                    })
                    .get_with(show, |op| {
                        resource_op!(op, PersonaDocs::Show).security_requirement("BearerToken")
                    })
                    .delete_with(remove, |op| {
                        resource_op!(op, PersonaDocs::Remove).security_requirement("BearerToken")
                    }),
                ),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<PersonaName>,
    Json(mut body): Json<SetPersona>,
) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(PersonaService::set(&context, &body).await?),
    ))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListPersonas>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(key): Path<ResourceKey<PersonaName>>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::get(&context, &GetPersona::builder().key(key).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::remove(&context, &RemovePersona::builder().name(name).build()).await?,
    ))
}
