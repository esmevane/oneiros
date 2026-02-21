use oneiros_model::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListAgentsOutcomes {
    #[outcome(message("No agents configured."))]
    NoAgents,

    #[outcome(message("Agents: {0:?}"))]
    Agents(Vec<AgentRecord>),
}
