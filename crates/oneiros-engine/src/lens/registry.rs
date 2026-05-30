use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArgType {
    String,
    Ref,
    Integer,
    Lens,
    /// A lens producing a set of names of the given kind. Bare symbols
    /// in this position compile to singleton name-lenses with this kind.
    LensOfNames(NameKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum NameKind {
    Agent,
    Texture,
    Level,
    Kind,
}

impl NameKind {
    pub(crate) fn describe(self) -> &'static str {
        match self {
            NameKind::Agent => "agent",
            NameKind::Texture => "texture",
            NameKind::Level => "level",
            NameKind::Kind => "kind",
        }
    }
}

pub(crate) trait NameRegistry {
    fn knows(&self, kind: NameKind, name: &Identifier) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultType {
    Entities,
    Events,
}

impl ResultType {
    pub(crate) fn describe(self) -> &'static str {
        match self {
            ResultType::Entities => "entities",
            ResultType::Events => "events",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpecResultType {
    Of(ResultType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExecutorHint {
    SearchIndexText,
    GraphStep(StepKind),
    ChronicleBetween,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PredicateSpec {
    pub(crate) name: PredicateName,
    pub(crate) arg_types: Vec<ArgType>,
    pub(crate) result_type: SpecResultType,
    pub(crate) executor: ExecutorHint,
}

impl PredicateSpec {
    pub(crate) fn new(
        name: impl Into<PredicateName>,
        arg_types: impl IntoIterator<Item = ArgType>,
        result_type: SpecResultType,
        executor: ExecutorHint,
    ) -> Self {
        Self {
            name: name.into(),
            arg_types: arg_types.into_iter().collect(),
            result_type,
            executor,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Registry {
    predicates: HashMap<PredicateName, PredicateSpec>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with(mut self, spec: PredicateSpec) -> Self {
        self.predicates.insert(spec.name.clone(), spec);
        self
    }

    pub(crate) fn lookup(&self, name: &PredicateName) -> Option<&PredicateSpec> {
        self.predicates.get(name)
    }

    pub(crate) fn seed_default() -> Self {
        let entities = SpecResultType::Of(ResultType::Entities);
        let events = SpecResultType::Of(ResultType::Events);

        Self::new()
            .with(PredicateSpec::new(
                "agent",
                [ArgType::LensOfNames(NameKind::Agent)],
                entities,
                ExecutorHint::GraphStep(StepKind::SearchByAgent),
            ))
            .with(PredicateSpec::new(
                "texture",
                [ArgType::LensOfNames(NameKind::Texture)],
                entities,
                ExecutorHint::GraphStep(StepKind::SearchByTexture),
            ))
            .with(PredicateSpec::new(
                "level",
                [ArgType::LensOfNames(NameKind::Level)],
                entities,
                ExecutorHint::GraphStep(StepKind::SearchByLevel),
            ))
            .with(PredicateSpec::new(
                "kind",
                [ArgType::LensOfNames(NameKind::Kind)],
                entities,
                ExecutorHint::GraphStep(StepKind::SearchByKind),
            ))
            .with(PredicateSpec::new(
                "search",
                [ArgType::String],
                entities,
                ExecutorHint::SearchIndexText,
            ))
            .with(PredicateSpec::new(
                "events_for",
                [ArgType::Lens],
                events,
                ExecutorHint::GraphStep(StepKind::EventsFor),
            ))
            .with(PredicateSpec::new(
                "refs_from",
                [ArgType::Lens],
                entities,
                ExecutorHint::GraphStep(StepKind::RefsFrom),
            ))
            .with(PredicateSpec::new(
                "from",
                [ArgType::Lens],
                entities,
                ExecutorHint::GraphStep(StepKind::ConnectedFrom),
            ))
            .with(PredicateSpec::new(
                "to",
                [ArgType::Lens],
                entities,
                ExecutorHint::GraphStep(StepKind::ConnectedTo),
            ))
            .with(PredicateSpec::new(
                "descendants",
                [ArgType::Lens],
                entities,
                ExecutorHint::GraphStep(StepKind::Descendants),
            ))
            .with(PredicateSpec::new(
                "ancestors",
                [ArgType::Lens],
                entities,
                ExecutorHint::GraphStep(StepKind::Ancestors),
            ))
            .with(PredicateSpec::new(
                "within",
                [ArgType::Lens, ArgType::Integer],
                entities,
                ExecutorHint::GraphStep(StepKind::Within(0)),
            ))
            .with(PredicateSpec::new(
                "component",
                [ArgType::Lens],
                entities,
                ExecutorHint::GraphStep(StepKind::Component),
            ))
            .with(PredicateSpec::new(
                "between",
                [ArgType::Ref, ArgType::Ref],
                events,
                ExecutorHint::ChronicleBetween,
            ))
    }
}

impl ArgType {
    pub(crate) fn matches(&self, lens: &Lens) -> bool {
        match (self, lens) {
            (Self::String, Lens::String(_)) => true,
            (Self::Ref, Lens::Ref(_)) => true,
            (Self::Integer, Lens::Integer(_)) => true,
            (Self::Lens, lens) => matches!(
                lens,
                Lens::Predicate(_)
                    | Lens::Union(_, _)
                    | Lens::Intersection(_, _)
                    | Lens::Difference(_, _)
                    | Lens::Ref(_)
            ),
            (Self::LensOfNames(_), lens) => Self::lens_of_names_matches(lens),
            _ => false,
        }
    }

    fn lens_of_names_matches(lens: &Lens) -> bool {
        match lens {
            Lens::Symbol(_) => true,
            Lens::Union(left, right)
            | Lens::Intersection(left, right)
            | Lens::Difference(left, right) => {
                Self::lens_of_names_matches(left) && Self::lens_of_names_matches(right)
            }
            _ => false,
        }
    }

    pub(crate) fn describe(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Ref => "ref",
            Self::Integer => "integer",
            Self::Lens => "lens expression",
            Self::LensOfNames(NameKind::Agent) => "lens of agent names",
            Self::LensOfNames(NameKind::Texture) => "lens of texture names",
            Self::LensOfNames(NameKind::Level) => "lens of level names",
            Self::LensOfNames(NameKind::Kind) => "lens of resource kinds",
        }
    }
}
