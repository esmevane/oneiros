use aide::axum::routing::ApiMethodRouter;
use aide::transform::TransformOperation;
use kinded::Kind;
use schemars::JsonSchema;

use crate::*;

/// Per-request-struct metadata — carried by each variant of a resource enum.
pub(crate) trait ResourceRequestMeta {
    type Kind: Kind;
    type Domain: DomainDef<Kind = Self::Kind>;
    type Response: JsonSchema;

    const PATH: &'static str;
    const SUMMARY: &'static str;
    const DESCRIPTION: &'static str;
    const STATUS: u16 = 200;

    fn content() -> &'static str;

    fn resource_docs(tag: Tag, nickname: Label) -> ResourceDocs {
        ResourceDocs::builder()
            .tag(tag)
            .nickname(nickname)
            .summary(Self::SUMMARY)
            .description(Self::DESCRIPTION)
            .content(Self::content())
            .build()
    }

    fn transform<'t>(op: TransformOperation<'t>, nickname: Label) -> TransformOperation<'t> {
        let docs = Self::resource_docs(Self::Domain::tag(), nickname);
        op.id(docs.nickname.as_str())
            .tag(docs.tag.name.as_str())
            .summary(docs.summary.as_str())
            .description(docs.description.as_str())
            .response::<200, axum::Json<Self::Response>>()
    }

    fn route(nickname: Label) -> ApiMethodRouter<ServerState>;
}

/// Dispatch bridge — implemented by the owning enum (e.g. `ActorRequest`).
pub(crate) trait ResourceDispatch {
    type Kind: Kind;

    fn path_for(kind: Self::Kind) -> &'static str;
    fn summary_for(kind: Self::Kind) -> &'static str;
    fn description_for(kind: Self::Kind) -> &'static str;
    fn content_for(kind: Self::Kind) -> &'static str;
    fn route_for(kind: Self::Kind) -> ApiMethodRouter<ServerState>;
}
