use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use serde::Deserialize;

use crate::*;

pub struct ExperienceRouter;

impl ExperienceRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/experiences",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| {
                        resource_op!(op, ExperienceDocs::List).security_requirement("BearerToken")
                    })
                    .post_with(create, |op| {
                        resource_op!(op, ExperienceDocs::Create)
                            .security_requirement("BearerToken")
                            .response::<201, Json<ExperienceResponse>>()
                    }),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| {
                        resource_op!(op, ExperienceDocs::Show).security_requirement("BearerToken")
                    }),
                )
                // Local body structs (UpdateDescriptionBody, UpdateSensationBody) don't
                // implement OperationInput — use plain route() to skip schema generation.
                .route("/{id}/description", axum::routing::put(update_description))
                .route("/{id}/sensation", axum::routing::put(update_sensation)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct UpdateDescriptionBody {
    description: Description,
}

#[derive(Debug, Deserialize)]
struct UpdateSensationBody {
    sensation: SensationName,
}

async fn create(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Json(body): Json<CreateExperience>,
) -> Result<(StatusCode, Json<ExperienceResponse>), ExperienceError> {
    let response = ExperienceService::create(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtBookmark>,
    Query(params): Query<ListExperiences>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtBookmark>,
    Path(key): Path<ResourceKey<ExperienceId>>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::get(&scope, &GetExperience::builder_v1().key(key).build().into())
            .await?,
    ))
}

async fn update_description(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(id): Path<ExperienceId>,
    Json(body): Json<UpdateDescriptionBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::update_description(
            &scope,
            &mailbox,
            &UpdateExperienceDescription::builder_v1()
                .id(id)
                .description(body.description)
                .build()
                .into(),
        )
        .await?,
    ))
}

async fn update_sensation(
    scope: Scope<AtBookmark>,
    mailbox: Mailbox,
    Path(id): Path<ExperienceId>,
    Json(body): Json<UpdateSensationBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::update_sensation(
            &scope,
            &mailbox,
            &UpdateExperienceSensation::builder_v1()
                .id(id)
                .sensation(body.sensation)
                .build()
                .into(),
        )
        .await?,
    ))
}
