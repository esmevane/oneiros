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
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<PersonaName>,
    Json(body): Json<SetPersona>,
) -> Result<(StatusCode, Json<PersonaResponse>), PersonaError> {
    let SetPersona::V1(mut setting) = body;
    setting.name = name;
    let request = SetPersona::V1(setting);
    Ok((
        StatusCode::OK,
        Json(PersonaService::set(&scope, &mailbox, &request).await?),
    ))
}

async fn list(
    scope: Scope<AtBookmark>,
    Query(params): Query<ListPersonas>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(PersonaService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(key): Path<ResourceKey<PersonaName>>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::get(&scope, &GetPersona::builder_v1().key(key).build().into()).await?,
    ))
}

async fn remove(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(name): Path<PersonaName>,
) -> Result<Json<PersonaResponse>, PersonaError> {
    Ok(Json(
        PersonaService::remove(
            &scope,
            &mailbox,
            &RemovePersona::builder_v1().name(name).build().into(),
        )
        .await?,
    ))
}
