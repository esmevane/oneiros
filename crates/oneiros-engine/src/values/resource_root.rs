use std::collections::BTreeMap;

use aide::axum::{ApiRouter, routing::ApiMethodRouter};
use kinded::Kind;

use crate::*;

/// Pure resource metadata dispatch — on the kind enum.
/// Kind value -> ResourceMeta by value. Pure forwarding match.
/// Carries `type Root` to bridge to the domain's ResourceRoot, so
/// `resource_docs()` can be a default.
pub(crate) trait ResourceOpMeta: core::fmt::Display {
    type Root: ResourceRoot;

    fn meta(&self) -> ResourceMeta;

    /// Build `ResourceDocs` from meta() and the root's tag.
    /// Default — no per-domain hand-writing needed.
    fn resource_docs(&self) -> ResourceDocs {
        let meta = self.meta();
        ResourceDocs::builder()
            .tag(Self::Root::tag())
            .nickname(self.to_string())
            .summary(meta.summary)
            .description(meta.description)
            .content(meta.content)
            .build()
    }
}

/// Route definition for a resource — carries a function pointer that builds
/// the ApiMethodRouter, with all type-level info (Response, STATUS, inputs)
/// baked inside. Parallels ResourceMeta: a data struct returned by a fn.
pub(crate) struct ResourceRouteDef<K> {
    pub build: fn(K) -> ApiMethodRouter<ServerState>,
}

/// Route dispatch — on the kind enum.
/// Parallels ResourceOpMeta: kind value -> ResourceRouteDef by value.
/// Pure forwarding match.
pub(crate) trait ResourceOpRoute {
    fn route_def(&self) -> ResourceRouteDef<Self>
    where
        Self: Sized;
}

/// Domain entry point — carries domain identity and the consumer loops.
/// Auto-impls router() and skills() by iterating the kind enum and reading
/// from ResourceOpMeta (for data) and ResourceOpRoute (for routing).
pub(crate) trait ResourceRoot {
    type Kind: Kind + core::fmt::Display + ResourceOpMeta + ResourceOpRoute;

    const LABEL: &'static str;
    const PURPOSE: &'static str;

    fn tag() -> Tag {
        Tag::builder()
            .name(Self::LABEL)
            .description(Self::PURPOSE)
            .build()
    }

    fn router() -> ApiRouter<ServerState>
    where
        Self: Sized,
        Self::Kind: 'static,
    {
        let mut by_path: BTreeMap<&'static str, Vec<Self::Kind>> = BTreeMap::new();
        for kind in Self::Kind::all() {
            by_path.entry(kind.meta().path).or_default().push(*kind);
        }

        let mut inner = ApiRouter::<ServerState>::new();
        for (path, kinds) in by_path {
            let methods = kinds
                .into_iter()
                .map(|kind| {
                    let def = kind.route_def();
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
        Self::Kind: 'static,
    {
        Self::Kind::all()
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
