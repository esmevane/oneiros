use bon::Builder;

use crate::*;

/// Hints after listing — suggest search, note when there's more.
#[derive(Builder)]
pub(crate) struct ListingHints {
    pub(crate) agent: AgentName,
    #[builder(default)]
    pub(crate) has_more: bool,
}

impl ListingHints {
    pub(crate) fn hints(&self) -> Vec<Hint> {
        let agent = &self.agent;
        let mut hints = vec![Hint::inspect(
            format!("search {agent}"),
            "Search across everything in the project",
        )];

        if self.has_more {
            hints.push(Hint::inspect(
                format!("cognition list --agent {agent}"),
                "There are more items to explore",
            ));
        }

        hints
    }
}
