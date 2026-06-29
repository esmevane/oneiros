mod features;
mod model;
mod protocol;
mod repo;
mod service;
mod store;
mod view;

pub(crate) use features::*;
pub(crate) use model::*;
pub(crate) use operations::*;
pub(crate) use protocol::*;
pub(crate) use repo::*;
pub(crate) use service::*;
pub(crate) use store::*;
pub(crate) use view::*;

mod operations {
    use std::collections::BTreeMap;

    use aide::{
        axum::{
            ApiRouter,
            routing::{self, ApiMethodRouter},
        },
        transform::TransformOperation,
    };
    use axum::Json;
    use kinded::Kind;
    use schemars::JsonSchema;

    use crate::*;

    enum RouteMethod {
        Post,
        Get,
    }

    trait ResourceRouteDocs {
        fn resource_docs(&self) -> ResourceDocs;
    }

    trait ResourceRoute: ResourceRouteDocs {
        type Response: JsonSchema;

        const STATUS: u16 = 200;
        const METHOD: RouteMethod;

        fn transform<'a>(&self, op: TransformOperation<'a>) -> TransformOperation<'a> {
            op.response::<{ Self::STATUS }, Json<Self::Response>>()
        }

        fn finalize_transform<'a>(&self, op: TransformOperation<'a>) -> TransformOperation<'a> {
            let docs = self.resource_docs();

            self.transform(docs.transform(op))
        }

        fn route(&self) -> ApiMethodRouter<ServerState> {
            let handle = |op| self.finalize_transform(op)
            match Self::METHOD {
                RouteMethod::Post => routing::post_with(Self::handler, handle),
                RouteMethod::Get => routing::get_with(Self::handler, handle)
            }
        }
    }

    trait ResourceRootMeta<T: Kind> {
        const LABEL: &'static str;
        const PURPOSE: &'static str;
    }

    trait ResourceOpMeta {
        fn meta(&self) -> ResourceMeta;
    }

    impl ResourceOpMeta for ActorRequestType {
        fn meta(&self) -> ResourceMeta {
            match self {
                Self::CreateActor => CreateActor::meta(),
                Self::GetActor => GetActor::meta(),
                Self::ListActors => ListActors::meta(),
            }
        }
    }

    trait ResourceRootDocs<T: Kind + ResourceOpMeta + core::fmt::Display>: ResourceRootMeta<T> {
        fn tag() -> Tag {
            Tag::builder()
                .name(Self::LABEL)
                .description(Self::PURPOSE)
                .build()
        }

        fn skills() -> Vec<Skill>
        where
            T: 'static,
        {
            T::all()
                .iter()
                .map(|kind| {
                    let meta = kind.meta();
                    Skill::builder()
                        .name(kind.to_string())
                        .content(meta.content)
                        .build()
                })
                .collect()
        }
    }

    trait ResourceRootRouter<T: Kind + ResourceOpMeta + core::fmt::Display>:
        ResourceRootMeta<T>
    {
        fn tag() -> Tag {
            Tag::builder()
                .name(Self::LABEL)
                .description(Self::PURPOSE)
                .build()
        }

        fn skills() -> Vec<Skill>
        where
            T: 'static,
        {
            T::all()
                .iter()
                .map(|kind| {
                    let meta = kind.meta();
                    Skill::builder()
                        .name(kind.to_string())
                        .content(meta.content)
                        .build()
                })
                .collect()
        }
    }

    /// Single point of entry for the actor domain.
    /// Carries domain identity (LABEL, PURPOSE, tag) and the consumer loops
    /// (router, skills) that iterate the kind enum and read from `meta()`.
    /// The `http_handler` match is the hand-written aide consumer — type-level
    /// calls (response types, input types) that can't be expressed as data.
    pub(crate) struct ActorOperations;

    impl ActorOperations {
        pub(crate) const LABEL: &'static str = "actors";
        pub(crate) const PURPOSE: &'static str = "Manage actors within a tenant";

        pub(crate) fn tag() -> Tag {
            Tag::builder()
                .name(Self::LABEL)
                .description(Self::PURPOSE)
                .build()
        }

        pub(crate) fn router() -> ApiRouter<ServerState> {
            let mut by_path: BTreeMap<&'static str, Vec<ActorRequestType>> = BTreeMap::new();
            for kind in ActorRequestType::all() {
                by_path.entry(kind.meta().path).or_default().push(*kind);
            }

            let mut inner = ApiRouter::<ServerState>::new();
            for (path, kinds) in by_path {
                let methods = kinds
                    .into_iter()
                    .map(Self::http_handler)
                    .reduce(|current, other| current.merge(other))
                    .unwrap();
                inner = inner.api_route(path, methods);
            }

            ApiRouter::new().nest(&format!("/{}", Self::LABEL), inner)
        }

        pub(crate) fn skills() -> Vec<Skill> {
            ActorRequestType::all()
                .iter()
                .map(|kind| {
                    let meta = kind.meta();
                    Skill::builder()
                        .name(kind.to_string())
                        .content(meta.content)
                        .build()
                })
                .collect()
        }

        fn http_handler(kind: ActorRequestType) -> ApiMethodRouter<ServerState> {
            match kind {
                ActorRequestType::CreateActor => routing::post_with(CreateActor::handler, |op| {
                    resource_op!(op, kind).response::<201, Json<ActorCreatedResponse>>()
                }),
                ActorRequestType::GetActor => routing::get_with(GetActor::handler, |op| {
                    resource_op!(op, kind)
                        .input::<IdPathParam<ActorId>>()
                        .response::<200, Json<ActorFoundResponse>>()
                }),
                ActorRequestType::ListActors => routing::get_with(ListActors::handler, |op| {
                    resource_op!(op, kind).response::<200, Json<ActorsResponse>>()
                }),
            }
        }
    }
}
