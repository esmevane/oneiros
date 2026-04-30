use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub struct TenantRouter;

impl TenantRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new().nest(
            "/tenants",
            ApiRouter::new()
                .api_route(
                    "/",
                    routing::get_with(list, |op| resource_op!(op, TenantDocs::List)).post_with(
                        create,
                        |op| {
                            resource_op!(op, TenantDocs::Create)
                                .response::<201, Json<TenantResponse>>()
                        },
                    ),
                )
                .api_route(
                    "/{id}",
                    routing::get_with(show, |op| resource_op!(op, TenantDocs::Show)),
                ),
        )
    }
}

async fn create(
    context: HostLog,
    Json(body): Json<CreateTenant>,
) -> Result<(StatusCode, Json<TenantResponse>), TenantError> {
    let response = TenantService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: HostLog,
    Query(params): Query<ListTenants>,
) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(TenantService::list(&context, &params).await?))
}

async fn show(
    context: HostLog,
    Path(key): Path<ResourceKey<TenantId>>,
) -> Result<Json<TenantResponse>, TenantError> {
    Ok(Json(
        TenantService::get(&context, &GetTenant::builder_v1().key(key).build().into()).await?,
    ))
}
