use crate::*;

/// All known hint contexts. Each variant carries a named type that
/// knows what data it needs and how to produce its hints.
pub(crate) enum HintSet {
    None,
    Wake(WakeHints),
    Reflect(ReflectHints),
    CognitionAdded(CognitionAddedHints),
    Mutation(MutationHints),
    Listing(ListingHints),
    AgentCreated(AgentCreatedHints),
    Vocabulary(VocabularyHints),
}

impl HintSet {
    pub(crate) fn wake(inner: WakeHints) -> Self {
        Self::Wake(inner)
    }

    pub(crate) fn reflect(inner: ReflectHints) -> Self {
        Self::Reflect(inner)
    }

    pub(crate) fn cognition_added(inner: CognitionAddedHints) -> Self {
        Self::CognitionAdded(inner)
    }

    pub(crate) fn mutation(inner: MutationHints) -> Self {
        Self::Mutation(inner)
    }

    pub(crate) fn listing(inner: ListingHints) -> Self {
        Self::Listing(inner)
    }

    pub(crate) fn agent_created(inner: AgentCreatedHints) -> Self {
        Self::AgentCreated(inner)
    }

    pub(crate) fn vocabulary(inner: VocabularyHints) -> Self {
        Self::Vocabulary(inner)
    }

    pub(crate) fn hints(&self) -> Vec<Hint> {
        match self {
            HintSet::None => Vec::new(),
            HintSet::Wake(h) => h.hints(),
            HintSet::Reflect(h) => h.hints(),
            HintSet::CognitionAdded(h) => h.hints(),
            HintSet::Mutation(h) => h.hints(),
            HintSet::Listing(h) => h.hints(),
            HintSet::AgentCreated(h) => h.hints(),
            HintSet::Vocabulary(h) => h.hints(),
        }
    }
}
