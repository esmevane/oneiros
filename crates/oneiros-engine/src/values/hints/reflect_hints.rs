use bon::Builder;

use crate::*;

/// Hints after reflecting — consolidate or deepen.
#[derive(Builder)]
pub struct ReflectHints {
    pub agent: AgentName,
}

impl ReflectHints {
    pub fn hints(&self) -> Vec<Hint> {
        let agent = &self.agent;
        vec![
            Hint::suggest(
                format!("memory add {agent} session \"...\""),
                "Consolidate this reflection into memory",
            ),
            Hint::suggest(
                format!("experience create {agent} \"...\""),
                "Mark this as a meaningful moment",
            ),
            Hint::inspect(
                format!("cognition list {agent}"),
                "Review the thought stream that led here",
            ),
        ]
    }
}
