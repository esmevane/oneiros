use oneiros_model::AgentName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveAgentOutcomes {
    #[outcome(message("Agent '{0}' removed."))]
    AgentRemoved(AgentName),
}
