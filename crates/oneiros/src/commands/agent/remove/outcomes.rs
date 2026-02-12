use oneiros_model::AgentName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum RemoveAgentOutcomes {
    #[outcome(message("Agent '{0}' removed."))]
    AgentRemoved(AgentName),
}
