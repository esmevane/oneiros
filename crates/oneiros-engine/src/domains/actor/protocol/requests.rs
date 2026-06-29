use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
};
use reqwest::StatusCode;

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateActor {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)] pub(crate) tenant_id: TenantId,
            #[builder(into)] pub(crate) name: ActorName,
        }
    }
}

impl CreateActor {
    pub(crate) const fn meta() -> ResourceMeta {
        ResourceMeta {
            path: "/",
            summary: "Create an actor",
            description: "Register a new actor under the current tenant.",
            content: include_str!("../features/skills/create.md"),
            status: 201,
        }
    }

    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<CreateActor>,
    ) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
        let response = ActorService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetActor {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ActorId>,
        }
    }
}

impl GetActor {
    pub(crate) const fn meta() -> ResourceMeta {
        ResourceMeta {
            path: "/{id}",
            summary: "Get an actor",
            description: "Look up a specific actor by ID.",
            content: include_str!("../features/skills/get.md"),
            status: 200,
        }
    }

    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<ActorId>>,
    ) -> Result<Json<ActorResponse>, ActorError> {
        Ok(Json(
            ActorService::get(&scope, &GetActor::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListActors {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListActors {
    pub(crate) const fn meta() -> ResourceMeta {
        ResourceMeta {
            path: "/",
            summary: "List actors",
            description: "List all actors for a tenant.",
            content: include_str!("../features/skills/list.md"),
            status: 200,
        }
    }

    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Query(params): Query<ListActors>,
    ) -> Result<Json<ActorResponse>, ActorError> {
        Ok(Json(ActorService::list(&scope, &params).await?))
    }
}

resource_requests! {
    CreateActor => |this, client| { client.post("/actors", this).await },
    GetActor => |this, client| {
        let GetActor::V1(lookup) = this;
        client.get(&format!("/actors/{}", lookup.key)).await
    },
    ListActors => |this, client| {
        let ListActors::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/actors?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ActorRequestType, display = "kebab-case")]
pub(crate) enum ActorRequest {
    CreateActor(CreateActor),
    GetActor(GetActor),
    ListActors(ListActors),
}

impl ActorRequestType {
    /// Dispatch from a kind to the inner struct's `ResourceMeta`.
    /// The match arms are the irreducible bridge — each variant maps to one
    /// inner struct's `meta()` call. A derive macro could generate this.
    pub(crate) fn meta(&self) -> ResourceMeta {
        match self {
            Self::CreateActor => CreateActor::meta(),
            Self::GetActor => GetActor::meta(),
            Self::ListActors => ListActors::meta(),
        }
    }

    /// Build `ResourceDocs` for this kind from the inner struct's metadata.
    /// Consumed by `resource_op!` and the docs inventory.
    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let meta = self.meta();
        ResourceDocs::builder()
            .tag(ActorOperations::tag())
            .nickname(self.to_string())
            .summary(meta.summary)
            .description(meta.description)
            .content(meta.content)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ActorRequestType::CreateActor, "create-actor"),
            (ActorRequestType::GetActor, "get-actor"),
            (ActorRequestType::ListActors, "list-actors"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }

    #[test]
    fn meta_dispatch_returns_correct_metadata() {
        assert_eq!(ActorRequestType::CreateActor.meta().status, 201);
        assert_eq!(ActorRequestType::CreateActor.meta().path, "/");
        assert_eq!(ActorRequestType::GetActor.meta().path, "/{id}");
        assert_eq!(ActorRequestType::ListActors.meta().summary, "List actors");
    }
}
