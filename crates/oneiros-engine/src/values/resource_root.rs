use std::collections::BTreeMap;

use aide::axum::{
    ApiRouter,
    routing::{self, ApiMethodRouter},
};
use aide::transform::TransformOperation;
use kinded::{Kind, Kinded};

use crate::*;

/// Type alias to extract the Kind from a leaf that implements ResourceLeafKind.
/// Used by macros to avoid complex projection syntax.
pub(crate) type __KindOf<L> = <L as ResourceLeafKind>::Kind;

/// Domain entry point — carries domain identity and the dispatch methods.
/// The dispatch (meta_for, route_def_for) is the irreducible bridge: kind
/// value -> inner struct's methods. A declarative macro generates this from
/// the enum declaration.
///
/// router() and skills() are auto-impl'd by iterating the kind enum and
/// reading from meta_for() and route_def_for().
pub(crate) trait ResourceRoot: Kinded
where
    <Self as Kinded>::Kind: Kind + core::fmt::Display,
{
    const LABEL: &'static str;
    const PURPOSE: &'static str;

    fn meta_for(kind: <Self as Kinded>::Kind) -> ResourceMeta;
    fn route_def_for(kind: <Self as Kinded>::Kind) -> ResourceRouteDef<<Self as Kinded>::Kind>;

    fn tag() -> Tag {
        Tag::builder()
            .name(Self::LABEL)
            .description(Self::PURPOSE)
            .build()
    }

    /// Build `ResourceDocs` for a kind from meta_for() and tag().
    fn resource_docs(kind: <Self as Kinded>::Kind) -> ResourceDocs {
        let meta = Self::meta_for(kind);
        ResourceDocs::builder()
            .tag(Self::tag())
            .nickname(kind.to_string())
            .summary(meta.summary)
            .description(meta.description)
            .content(meta.content)
            .build()
    }

    fn router() -> ApiRouter<ServerState>
    where
        Self: Sized,
        <Self as Kinded>::Kind: 'static,
    {
        let mut by_path: BTreeMap<&'static str, Vec<<Self as Kinded>::Kind>> = BTreeMap::new();
        for kind in <Self as Kinded>::Kind::all() {
            by_path
                .entry(Self::meta_for(*kind).path)
                .or_default()
                .push(*kind);
        }

        let mut inner = ApiRouter::<ServerState>::new();
        for (path, kinds) in by_path {
            let methods = kinds
                .into_iter()
                .map(|kind| {
                    let def = Self::route_def_for(kind);
                    (def.build)(kind)
                })
                .reduce(|a, b| a.merge(b))
                .unwrap();
            inner = inner.api_route(path, methods);
        }

        ApiRouter::new().nest(&format!("/{}", Self::LABEL), inner)
    }

    fn skills() -> Vec<Skill>
    where
        Self: Sized,
        <Self as Kinded>::Kind: 'static,
    {
        <Self as Kinded>::Kind::all()
            .iter()
            .map(|kind| {
                let meta = Self::meta_for(*kind);
                Skill::builder()
                    .name(kind.to_string())
                    .content(meta.content)
                    .build()
            })
            .collect()
    }
}

/// Route definition for a resource — carries a function pointer that builds
/// the ApiMethodRouter, with all type-level info (Response, STATUS, inputs)
/// baked inside. Parallels ResourceMeta: a data struct returned by a fn.
pub(crate) struct ResourceRouteDef<K> {
    pub build: fn(K) -> ApiMethodRouter<ServerState>,
}

/// HTTP method for a resource route.
#[derive(Clone, Copy)]
pub(crate) enum ResourceMethod {
    Get,
    Post,
}

impl ResourceMethod {
    /// Build an ApiMethodRouter for this method, given a handler and transform.
    /// The transform closure receives an op already set up with docs.
    pub(crate) fn router<H, I, O, T, F>(
        self,
        handler: H,
        transform: F,
    ) -> ApiMethodRouter<ServerState>
    where
        H: axum::handler::Handler<T, ServerState> + aide::operation::OperationHandler<I, O>,
        I: aide::operation::OperationInput,
        O: aide::operation::OperationOutput,
        T: 'static,
        F: FnOnce(
            aide::transform::TransformOperation<'_>,
        ) -> aide::transform::TransformOperation<'_>,
    {
        match self {
            Self::Get => routing::get_with(handler, transform),
            Self::Post => routing::post_with(handler, transform),
        }
    }
}

/// Leaf-level metadata contract. Each inner struct (e.g. CreateActor)
/// implements this to provide its ResourceMeta.
pub(crate) trait ResourceLeafMeta {
    fn meta() -> ResourceMeta;
}

/// Leaf-level routing contract. Each inner struct (e.g. CreateActor)
/// implements this to provide its ResourceRouteDef.
pub(crate) trait ResourceLeafRoute: ResourceLeafKind {
    fn route_def() -> ResourceRouteDef<<Self as ResourceLeafKind>::Kind>;
}

/// Leaf-level kind contract. Each inner struct knows its kind via Kinded.
/// This bridges the leaf to the kind enum for route_def's return type.
/// Carries `type Root` to bridge to the domain's ResourceRoot, so
/// route_def can call ResourceRoot::resource_docs().
pub(crate) trait ResourceLeafKind
where
    <Self::Root as Kinded>::Kind: core::fmt::Display,
{
    type Kind: Kind + core::fmt::Display;
    type Root: ResourceRoot;
}
