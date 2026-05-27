use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArgType {
    Symbol,
    SymbolOf(NameKind),
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NameKind {
    Agent,
    Texture,
    Level,
}

impl NameKind {
    pub(crate) fn describe(self) -> &'static str {
        match self {
            NameKind::Agent => "agent",
            NameKind::Texture => "texture",
            NameKind::Level => "level",
        }
    }
}

pub(crate) trait NameRegistry {
    fn knows(&self, kind: NameKind, name: &Identifier) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultType {
    Entities,
}

impl ResultType {
    pub(crate) fn describe(self) -> &'static str {
        match self {
            ResultType::Entities => "entities",
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
    SearchIndexFacet(SearchFacet),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchFacet {
    Kind,
    Agent,
    Texture,
    Level,
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

        Self::new()
            .with(PredicateSpec::new(
                "agent",
                [ArgType::SymbolOf(NameKind::Agent)],
                entities,
                ExecutorHint::SearchIndexFacet(SearchFacet::Agent),
            ))
            .with(PredicateSpec::new(
                "texture",
                [ArgType::SymbolOf(NameKind::Texture)],
                entities,
                ExecutorHint::SearchIndexFacet(SearchFacet::Texture),
            ))
            .with(PredicateSpec::new(
                "level",
                [ArgType::SymbolOf(NameKind::Level)],
                entities,
                ExecutorHint::SearchIndexFacet(SearchFacet::Level),
            ))
            .with(PredicateSpec::new(
                "kind",
                [ArgType::Symbol],
                entities,
                ExecutorHint::SearchIndexFacet(SearchFacet::Kind),
            ))
            .with(PredicateSpec::new(
                "search",
                [ArgType::String],
                entities,
                ExecutorHint::SearchIndexText,
            ))
    }
}

impl ArgType {
    pub(crate) fn matches(&self, lens: &Lens) -> bool {
        matches!(
            (self, lens),
            (Self::Symbol, Lens::Symbol(_))
                | (Self::SymbolOf(_), Lens::Symbol(_))
                | (Self::String, Lens::String(_))
        )
    }

    pub(crate) fn describe(&self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::SymbolOf(NameKind::Agent) => "agent symbol",
            Self::SymbolOf(NameKind::Texture) => "texture symbol",
            Self::SymbolOf(NameKind::Level) => "level symbol",
            Self::String => "string",
        }
    }
}
