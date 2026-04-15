use bon::Builder;

use crate::*;

/// Hints after adding a cognition — suggest reflecting or browsing.
#[derive(Builder)]
pub struct CognitionAddedHints {
    pub agent: AgentName,
    pub ref_token: RefToken,
}

impl CognitionAddedHints {
    pub fn hints(&self) -> Vec<Hint> {
        let agent = &self.agent;
        let ref_token = &self.ref_token;
        vec![
            Hint::suggest(format!("reflect {agent}"), "Pause on something significant"),
            Hint::inspect(
                format!("cognition list {agent}"),
                "Browse your full thought stream",
            ),
            Hint::suggest(
                format!("connection create {ref_token} <target>"),
                "Draw a line between related things",
            ),
        ]
    }
}
