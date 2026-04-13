use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};

use crate::*;

pub(crate) struct TenantRouter;

impl TenantRouter {
    pub(crate) fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/tenants",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show)),
        )
    }
}

async fn create(
    context: SystemContext,
    Json(body): Json<CreateTenant>,
) -> Result<(StatusCode, Json<TenantResponse>), TenantError> {
    let response = TenantService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: SystemContext,
    Query(params): Query<ListTenants>,
) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(TenantService::list(&context, &params).await?))
}

async fn show(
    context: SystemContext,
    Path(id): Path<TenantId>,
) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(
        TenantService::get(&context, &GetTenant::builder().id(id).build()).await?,
    ))
}
