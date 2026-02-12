use oneiros_model::AgentName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum UpdateAgentOutcomes {
    #[outcome(message("Agent '{0}' updated."))]
    AgentUpdated(AgentName),
}
