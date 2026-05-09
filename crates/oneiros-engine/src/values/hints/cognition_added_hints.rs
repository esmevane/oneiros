use bon::Builder;

use crate::*;

/// Hints after adding a cognition — suggest reflecting or browsing.
#[derive(Builder)]
pub(crate) struct CognitionAddedHints {
    pub(crate) agent: AgentName,
    pub(crate) ref_token: RefToken,
}

impl CognitionAddedHints {
    pub(crate) fn hints(&self) -> Vec<Hint> {
        let agent = &self.agent;
        let ref_token = &self.ref_token;
        vec![
            Hint::suggest(format!("reflect {agent}"), "Pause on something significant"),
            Hint::inspect(
                format!("cognition list --agent {agent}"),
                "Browse your full thought stream",
            ),
            Hint::suggest(
                format!("connection create <nature> {ref_token} <target>"),
                "Draw a line between related things",
            ),
        ]
    }
}
