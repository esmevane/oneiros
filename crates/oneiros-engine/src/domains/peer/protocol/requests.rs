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
        summary: "Add a peer",
        description: "Register a remote host as a peer. Provide an oneiros:// URI to add a remote peer with ticket-based auth, or a plain address for a follow peer.",
        content: include_str!("../features/skills/add.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                AddPeer::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<201, Json<PeerAddedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum AddPeer {
        #[derive(clap::Args)]
        V1 => {
            #[arg(id = "peer_address")]
            pub(crate) address: String,
            #[arg(long)]
            pub(crate) name: Option<String>,
        }
    }
}

impl AddPeer {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<AddPeer>,
    ) -> Result<(StatusCode, Json<PeerResponse>), PeerError> {
        let response = PeerService::add(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Get a peer",
        description: "Look up the connection details for a specific peer.",
        content: include_str!("../features/skills/get.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetPeer::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").input::<IdPathParam<PeerId>>().response::<200, Json<PeerFoundResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetPeer {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<PeerId>,
        }
    }
}

impl GetPeer {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<PeerId>>,
    ) -> Result<Json<PeerResponse>, PeerError> {
        Ok(Json(
            PeerService::get(&scope, &GetPeer::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Remove a peer",
        description: "Deregister a peer host.",
        content: include_str!("../features/skills/remove.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                RemovePeer::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<PeerRemovedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum RemovePeer {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: PeerId,
        }
    }
}

impl RemovePeer {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Path(id): Path<PeerId>,
    ) -> Result<Json<PeerResponse>, PeerError> {
        Ok(Json(
            PeerService::remove(
                &scope,
                &mailbox,
                &RemovePeer::builder_v1().id(id).build().into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List peers",
        description: "List all known peer hosts.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListPeers::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<PeersResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListPeers {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListPeers {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Query(params): Query<ListPeers>,
    ) -> Result<Json<PeerResponse>, PeerError> {
        Ok(Json(PeerService::list(&scope, &params).await?))
    }
}

resource_requests! {
    AddPeer => |this, client| {
        client.post("/peers", this).await
    },
    GetPeer => |this, client| {
        let GetPeer::V1(lookup) = this;
        client.get(&format!("/peers/{}", lookup.key)).await
    },
    RemovePeer => |this, client| {
        let RemovePeer::V1(removal) = this;
        client.delete(&format!("/peers/{}", removal.id)).await
    },
    ListPeers => |this, client| {
        let ListPeers::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/peers?{query}")).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PeerRequestType, display = "kebab-case")]
pub(crate) enum PeerRequest {
    AddPeer(AddPeer),
    GetPeer(GetPeer),
    RemovePeer(RemovePeer),
    ListPeers(ListPeers),
}

resource_root! {
    PeerRequest => {
        label: "peers",
        purpose: "Manage peer connections for distribution",
        operations: [AddPeer, GetPeer, RemovePeer, ListPeers],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (PeerRequestType::AddPeer, "add-peer"),
            (PeerRequestType::GetPeer, "get-peer"),
            (PeerRequestType::RemovePeer, "remove-peer"),
            (PeerRequestType::ListPeers, "list-peers"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
