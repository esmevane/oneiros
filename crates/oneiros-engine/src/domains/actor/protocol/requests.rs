use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use aide::axum::routing::{self, ApiMethodRouter};
use aide::transform::TransformOperation;
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

impl ResourceRequestMeta for CreateActor {
    type Kind = ActorRequestType;
    type Domain = ActorOperations;
    type Response = ActorCreatedResponse;

    const PATH: &'static str = "/";
    const SUMMARY: &'static str = "Create an actor";
    const DESCRIPTION: &'static str = "Register a new actor under the current tenant.";
    const STATUS: u16 = 201;

    fn content() -> &'static str {
        include_str!("../features/skills/create.md")
    }

    fn transform<'t>(op: TransformOperation<'t>, nickname: Label) -> TransformOperation<'t> {
        let docs = Self::resource_docs(Self::Domain::tag(), nickname);
        op.id(docs.nickname.as_str())
            .tag(docs.tag.name.as_str())
            .summary(docs.summary.as_str())
            .description(docs.description.as_str())
            .response::<{ Self::STATUS }, Json<Self::Response>>()
    }
    fn route(nickname: Label) -> ApiMethodRouter<ServerState> {
        routing::post_with(Self::handler, move |op| Self::transform(op, nickname))
    }
}

impl ResourceRequestMeta for GetActor {
    type Kind = ActorRequestType;
    type Domain = ActorOperations;
    type Response = ActorFoundResponse;
    const PATH: &'static str = "/{id}";
    const SUMMARY: &'static str = "Get an actor";
    const DESCRIPTION: &'static str = "Look up a specific actor by ID.";
    fn content() -> &'static str {
        include_str!("../features/skills/get.md")
    }
    fn transform<'t>(op: TransformOperation<'t>, nickname: Label) -> TransformOperation<'t> {
        let docs = Self::resource_docs(Self::Domain::tag(), nickname);
        op.id(docs.nickname.as_str())
            .tag(docs.tag.name.as_str())
            .summary(docs.summary.as_str())
            .description(docs.description.as_str())
            .input::<IdPathParam<ActorId>>()
            .response::<200, Json<ActorFoundResponse>>()
    }
    fn route(nickname: Label) -> ApiMethodRouter<ServerState> {
        routing::get_with(Self::handler, move |op| Self::transform(op, nickname))
    }
}

impl ResourceRequestMeta for ListActors {
    type Kind = ActorRequestType;
    type Domain = ActorOperations;
    type Response = ActorsResponse;
    const PATH: &'static str = "/";
    const SUMMARY: &'static str = "List actors";
    const DESCRIPTION: &'static str = "List all actors for a tenant.";
    fn content() -> &'static str {
        include_str!("../features/skills/list.md")
    }
    fn route(nickname: Label) -> ApiMethodRouter<ServerState> {
        routing::get_with(Self::handler, move |op| Self::transform(op, nickname))
    }
}

impl ResourceDispatch for ActorRequest {
    type Kind = ActorRequestType;

    fn path_for(kind: ActorRequestType) -> &'static str {
        match kind {
            ActorRequestType::CreateActor => <CreateActor as ResourceRequestMeta>::PATH,
            ActorRequestType::GetActor => <GetActor as ResourceRequestMeta>::PATH,
            ActorRequestType::ListActors => <ListActors as ResourceRequestMeta>::PATH,
        }
    }

    fn summary_for(kind: ActorRequestType) -> &'static str {
        match kind {
            ActorRequestType::CreateActor => <CreateActor as ResourceRequestMeta>::SUMMARY,
            ActorRequestType::GetActor => <GetActor as ResourceRequestMeta>::SUMMARY,
            ActorRequestType::ListActors => <ListActors as ResourceRequestMeta>::SUMMARY,
        }
    }

    fn description_for(kind: ActorRequestType) -> &'static str {
        match kind {
            ActorRequestType::CreateActor => <CreateActor as ResourceRequestMeta>::DESCRIPTION,
            ActorRequestType::GetActor => <GetActor as ResourceRequestMeta>::DESCRIPTION,
            ActorRequestType::ListActors => <ListActors as ResourceRequestMeta>::DESCRIPTION,
        }
    }

    fn content_for(kind: ActorRequestType) -> &'static str {
        match kind {
            ActorRequestType::CreateActor => <CreateActor as ResourceRequestMeta>::content(),
            ActorRequestType::GetActor => <GetActor as ResourceRequestMeta>::content(),
            ActorRequestType::ListActors => <ListActors as ResourceRequestMeta>::content(),
        }
    }

    fn route_for(kind: ActorRequestType) -> ApiMethodRouter<ServerState> {
        let nickname: Label = kind.to_string().into();
        match kind {
            ActorRequestType::CreateActor => <CreateActor as ResourceRequestMeta>::route(nickname),
            ActorRequestType::GetActor => <GetActor as ResourceRequestMeta>::route(nickname),
            ActorRequestType::ListActors => <ListActors as ResourceRequestMeta>::route(nickname),
        }
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
}
