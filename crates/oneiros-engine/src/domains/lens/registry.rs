use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArgType {
    Symbol,
    String,
    Ref,
    Integer,
    Lens,
}

/// The set a lens evaluates to, resolved against the registry.
///
/// `Events` are entries in the event log (chronicle walks, between-roots);
/// `Entities` are projected records (cognitions, memories, connections, agents).
/// The distinction matters at set-operator boundaries — combining the two is a
/// type error in c1's strict mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultType {
    Events,
    Entities,
}

impl ResultType {
    pub(crate) fn describe(self) -> &'static str {
        match self {
            ResultType::Events => "events",
            ResultType::Entities => "entities",
        }
    }
}

/// What a [`PredicateSpec`] declares about its result type.
///
/// `Of(_)` is a concrete declaration. `InheritsFromArg(i)` defers to the
/// resolved type of the predicate's `i`th argument — used by combinators like
/// `recent(set, n)` whose output matches their input set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpecResultType {
    Of(ResultType),
    InheritsFromArg(usize),
}

/// Executor hint declares which engine capability would evaluate a predicate.
/// This is *intent*, not implementation — for now it surfaces what each
/// predicate expects, so we can compare against substrates that actually
/// exist. Future work: a real executor dispatch on these hints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Executor {
    /// `search_index` FTS5 `match` against full text — `search("text")`.
    SearchIndexText,
    /// `search_index` facet/column filter — existing facets are kind, agent,
    /// texture, level, sensation, persona. Covers most charter predicates.
    SearchIndexFacet(SearchFacet),
    /// Chronicle (HAMT) walk for time-/root-based ranges — `between(root:A, root:B)`.
    ChronicleWalk,
    /// Connection-graph traversal — `reachable(seed, depth)`.
    GraphWalk,
    /// Ordering + limiting on a result set — `recent(set, n)`.
    /// Push-down to `search_index` (it already orders by `created_at desc`)
    /// or post-process for graph/chronicle sources.
    Recency,
    /// Connections table — navigation of the explicit ref graph.
    /// One substrate, many operations (one-hop, closure, bounded, component).
    /// The specific operation is implied by the predicate name.
    ConnectionTable,
    /// Placeholder for predicates whose executor isn't decided yet.
    #[allow(dead_code)]
    Unspecified,
}

/// Existing facet columns on the `search_index` table. Cross-checked against
/// `domains/search/repo.rs::where_clause` and the facet collection loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchFacet {
    Kind,
    Agent,
    Texture,
    Level,
    Sensation,
    Persona,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PredicateSpec {
    pub(crate) name: PredicateName,
    pub(crate) arg_types: Vec<ArgType>,
    pub(crate) result_type: SpecResultType,
    pub(crate) executor: Executor,
}

impl PredicateSpec {
    pub(crate) fn new(
        name: impl Into<PredicateName>,
        arg_types: impl IntoIterator<Item = ArgType>,
        result_type: SpecResultType,
        executor: Executor,
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

    /// Charter's starting predicate set, narrowed to what the dream paper
    /// exercise pulled on. Refine arg types as the type system matures.
    pub(crate) fn seed_default() -> Self {
        let entities = SpecResultType::Of(ResultType::Entities);
        let events = SpecResultType::Of(ResultType::Events);
        let inherits_first = SpecResultType::InheritsFromArg(0);

        Self::new()
            .with(PredicateSpec::new(
                "agent",
                [ArgType::Symbol],
                entities,
                Executor::SearchIndexFacet(SearchFacet::Agent),
            ))
            .with(PredicateSpec::new(
                "texture",
                [ArgType::Symbol],
                entities,
                Executor::SearchIndexFacet(SearchFacet::Texture),
            ))
            .with(PredicateSpec::new(
                "level",
                [ArgType::Symbol],
                entities,
                Executor::SearchIndexFacet(SearchFacet::Level),
            ))
            .with(PredicateSpec::new(
                "kind",
                [ArgType::Symbol],
                entities,
                Executor::SearchIndexFacet(SearchFacet::Kind),
            ))
            .with(PredicateSpec::new(
                "search",
                [ArgType::String],
                entities,
                Executor::SearchIndexText,
            ))
            .with(PredicateSpec::new(
                "recent",
                [ArgType::Lens, ArgType::Integer],
                inherits_first,
                Executor::Recency,
            ))
            // Connection-graph navigation: one hop, directional.
            .with(PredicateSpec::new(
                "from",
                [ArgType::Lens],
                entities,
                Executor::ConnectionTable,
            ))
            .with(PredicateSpec::new(
                "to",
                [ArgType::Lens],
                entities,
                Executor::ConnectionTable,
            ))
            // Closure / reachability: transitive, directional or bounded.
            .with(PredicateSpec::new(
                "descendants",
                [ArgType::Lens],
                entities,
                Executor::ConnectionTable,
            ))
            .with(PredicateSpec::new(
                "ancestors",
                [ArgType::Lens],
                entities,
                Executor::ConnectionTable,
            ))
            .with(PredicateSpec::new(
                "within",
                [ArgType::Lens, ArgType::Integer],
                entities,
                Executor::ConnectionTable,
            ))
            .with(PredicateSpec::new(
                "component",
                [ArgType::Lens],
                entities,
                Executor::ConnectionTable,
            ))
            // Chronicle range — events between two roots in the HAMT log.
            .with(PredicateSpec::new(
                "between",
                [ArgType::Ref, ArgType::Ref],
                events,
                Executor::ChronicleWalk,
            ))
    }
}

impl ArgType {
    pub(crate) fn matches(&self, lens: &Lens) -> bool {
        match (self, lens) {
            (Self::Symbol, Lens::Symbol(_)) => true,
            (Self::String, Lens::String(_)) => true,
            (Self::Ref, Lens::Ref(_)) => true,
            (Self::Integer, Lens::Integer(_)) => true,
            (Self::Lens, _) => true,
            _ => false,
        }
    }

    pub(crate) fn describe(&self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::String => "string",
            Self::Ref => "ref",
            Self::Integer => "integer",
            Self::Lens => "lens",
        }
    }
}
