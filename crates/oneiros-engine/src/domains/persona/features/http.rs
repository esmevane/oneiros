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
    Json(body): Json<SetPersona>,
) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
    let SetPersona::V1(mut setting) = body;
    setting.name = name;
    let request = SetPersona::V1(setting);
    Ok((
        StatusCode::OK,
        Json(PersonaService::set(&context, &request).await?),
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
        PersonaService::get(&context, &GetPersona::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<PersonaName>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::remove(
            &context,
            &RemovePersona::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
