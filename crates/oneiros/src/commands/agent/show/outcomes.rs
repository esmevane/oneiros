use oneiros_model::Agent;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowAgentOutcomes {
    #[outcome(message("Agent '{}' (persona: {})\n  Description: {}\n  Prompt: {}", .0.name, .0.persona, .0.description, .0.prompt))]
    AgentDetails(Agent),
}
