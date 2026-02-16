use oneiros_model::Cognition;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowCognitionOutcomes {
    #[outcome(
        message("Cognition {}\n  Agent: {}\n  Texture: {}\n  Content: {}\n  Created: {}", .0.id, .0.agent_id, .0.texture, .0.content, .0.created_at),
        prompt("Does this connect to something? Trace it with `oneiros experience create <agent> <sensation> <description>`.")
    )]
    CognitionDetails(Cognition),
}
