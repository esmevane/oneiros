use bon::Builder;

use crate::*;

/// Hints after creating an agent — suggest waking or exploring.
#[derive(Builder)]
pub struct AgentCreatedHints {
    pub agent: AgentName,
}

impl AgentCreatedHints {
    pub fn hints(&self) -> Vec<Hint> {
        let agent = &self.agent;
        vec![
            Hint::follow_up(format!("wake {agent}"), "Start a session with this agent"),
            Hint::suggest(format!("dream {agent}"), "Restore full cognitive context"),
        ]
    }
}
