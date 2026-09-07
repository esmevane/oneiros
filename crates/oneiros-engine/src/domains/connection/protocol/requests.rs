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
        summary: "Create a connection",
        description: "Draw a typed relationship between two entities using a defined nature.",
        content: include_str!("../features/skills/create.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateConnection::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<201, Json<ConnectionCreatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum CreateConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) nature: NatureName,
            pub(crate) from_ref: RefToken,
            pub(crate) to_ref: RefToken,
        }
    }
}

impl CreateConnection {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Json(body): Json<CreateConnection>,
    ) -> Result<(StatusCode, Json<ConnectionResponse>), ConnectionError> {
        let response = ConnectionService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Get a connection",
        description: "Look up the details of a specific relationship by ID.",
        content: include_str!("../features/skills/show.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetConnection::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<IdPathParam<ConnectionId>>().response::<200, Json<ConnectionDetailsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ConnectionId>,
        }
    }
}

impl GetConnection {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(key): Path<ResourceKey<ConnectionId>>,
    ) -> Result<Json<ConnectionResponse>, ConnectionError> {
        Ok(Json(
            ConnectionService::get(&scope, &GetConnection::builder_v1().key(key).build().into())
                .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List connections",
        description: "List all relationships visible to the current project, optionally filtered by nature or entity.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListConnections::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<ConnectionsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListConnections {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) entity: Option<RefToken>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, entity is ignored and the lens
            /// drives selection end-to-end.
            #[arg(long)]
            #[builder(into)]
            pub(crate) lens: Option<String>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListConnections {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Query(params): Query<ListConnections>,
    ) -> Result<Json<ConnectionResponse>, ConnectionError> {
        Ok(Json(ConnectionService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Remove a connection",
        description: "Delete a relationship between entities, removing it from the graph.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemoveConnection::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<ConnectionRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemoveConnection {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: ConnectionId,
        }
    }
}

impl RemoveConnection {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
        Path(id): Path<ConnectionId>,
    ) -> Result<Json<ConnectionResponse>, ConnectionError> {
        Ok(Json(
            ConnectionService::remove(
                &scope,
                &mailbox,
                &RemoveConnection::builder_v1().id(id).build().into(),
            )
            .await?,
        ))
    }
}

resource_requests! {
    CreateConnection => |this, client| { client.post("/connections", this).await },
    GetConnection => |this, client| {
        let GetConnection::V1(lookup) = this;
        client.get(&format!("/connections/{}", lookup.key)).await
    },
    ListConnections => |this, client| {
        let ListConnections::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(entity) = &listing.entity {
            params.push(("entity", entity.to_string()));
        }

        if let Some(lens) = &listing.lens {
            params.push(("lens", lens.clone()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        client.get(&format!("/connections?{query}")).await
    },
    RemoveConnection => |this, client| {
        let RemoveConnection::V1(removal) = this;
        client.delete(&format!("/connections/{}", removal.id)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ConnectionRequestType, display = "kebab-case")]
pub(crate) enum ConnectionRequest {
    CreateConnection(CreateConnection),
    GetConnection(GetConnection),
    ListConnections(ListConnections),
    RemoveConnection(RemoveConnection),
}

resource_root! {
    ConnectionRequest => {
        label: "connections",
        purpose: "Draw and manage relationships between entities",
        operations: [CreateConnection, GetConnection, ListConnections, RemoveConnection],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ConnectionRequestType::CreateConnection, "create-connection"),
            (ConnectionRequestType::GetConnection, "get-connection"),
            (ConnectionRequestType::ListConnections, "list-connections"),
            (ConnectionRequestType::RemoveConnection, "remove-connection"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
