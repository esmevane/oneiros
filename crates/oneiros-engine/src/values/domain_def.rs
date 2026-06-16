use aide::axum::{ApiRouter, routing::ApiMethodRouter};
use kinded::Kind;
use std::collections::BTreeMap;

use crate::*;

pub(crate) trait DomainDef {
    type Kind: Kind;
    type Request: ResourceDispatch<Kind = Self::Kind>;

    const LABEL: &'static str;
    const PURPOSE: &'static str;

    fn resource_definition(given_kind: Self::Kind) -> Self;

    fn http_handler(&self) -> ApiMethodRouter<ServerState>;
    fn resource(&self) -> Self::Kind;

    fn content(&self) -> Content {
        Self::Request::content_for(self.resource()).into()
    }

    fn description(&self) -> Description {
        Self::Request::description_for(self.resource()).into()
    }

    fn summary(&self) -> Description {
        Self::Request::summary_for(self.resource()).into()
    }

    fn path(&self) -> &'static str {
        Self::Request::path_for(self.resource())
    }

    fn label(&self) -> Label
    where
        Self::Kind: core::fmt::Display,
    {
        self.resource().to_string().into()
    }

    fn resource_docs(&self) -> ResourceDocs
    where
        Self::Kind: core::fmt::Display,
    {
        ResourceDocs::builder()
            .tag(Self::tag())
            .nickname(self.label())
            .summary(self.summary())
            .description(self.description())
            .content(self.content())
            .build()
    }

    fn skill(&self) -> Skill
    where
        Self::Kind: core::fmt::Display,
    {
        Skill::builder()
            .name(self.resource().to_string())
            .content(self.content().to_string())
            .build()
    }

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
            let ops = Self::resource_definition(*kind);
            by_path.entry(ops.path()).or_default().push(*kind);
        }

        let mut inner = ApiRouter::<ServerState>::new();
        for (path, kinds) in by_path {
            let methods = kinds
                .into_iter()
                .map(|given_kind| Self::resource_definition(given_kind).http_handler())
                .reduce(|current, other| current.merge(other))
                .unwrap();
            inner = inner.api_route(path, methods);
        }

        ApiRouter::new().nest(&format!("/{}", Self::LABEL), inner)
    }

    fn skills() -> Vec<Skill>
    where
        Self: Sized,
        Self::Kind: core::fmt::Display + 'static,
    {
        let mut skills = vec![];

        for kind in Self::Kind::all() {
            skills.push(Self::resource_definition(*kind).skill())
        }

        skills
    }
}
