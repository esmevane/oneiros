use oneiros_model::AgentName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateAgentOutcomes {
    #[outcome(message("Agent '{0}' created."))]
    AgentCreated(AgentName),
}
