use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Create a tenant",
        description: "Register a new tenant project on this host.",
        content: include_str!("../features/skills/create.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateTenant::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<201, Json<TenantCreatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum CreateTenant {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: TenantName,
        }
    }
}

impl CreateTenant {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<CreateTenant>,
    ) -> Result<(StatusCode, Json<TenantResponse>), TenantError> {
        let response = TenantService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Show a tenant",
        description: "Retrieve details for a specific tenant project on this host.",
        content: include_str!("../features/skills/get.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetTenant::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.input::<IdPathParam<TenantId>>().response::<200, Json<TenantFoundResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetTenant {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TenantId>,
        }
    }
}

impl GetTenant {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<TenantId>>,
    ) -> Result<Json<TenantResponse>, TenantError> {
        Ok(Json(
            TenantService::get(&scope, &GetTenant::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List tenants",
        description: "See all tenant projects registered on this host.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListTenants::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<TenantsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListTenants {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListTenants {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Query(params): Query<ListTenants>,
    ) -> Result<Json<TenantResponse>, TenantError> {
        Ok(Json(TenantService::list(&scope, &params).await?))
    }
}

resource_requests! {
    CreateTenant => |this, client| {
        client.post("/tenants", this).await
    },
    GetTenant => |this, client| {
        let GetTenant::V1(lookup) = this;
        client.get(&format!("/tenants/{}", lookup.key)).await
    },
    ListTenants => |this, client| {
        let ListTenants::V1(listing) = this;
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset,);
        client.get(&format!("/tenants?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TenantRequestType, display = "kebab-case")]
pub(crate) enum TenantRequest {
    CreateTenant(CreateTenant),
    GetTenant(GetTenant),
    ListTenants(ListTenants),
}

resource_root! {
    TenantRequest => {
        label: "tenants",
        purpose: "Manage tenants on this host",
        operations: [CreateTenant, GetTenant, ListTenants],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (TenantRequestType::CreateTenant, "create-tenant"),
            (TenantRequestType::GetTenant, "get-tenant"),
            (TenantRequestType::ListTenants, "list-tenants"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
