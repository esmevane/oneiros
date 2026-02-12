use oneiros_model::Cognition;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ShowCognitionOutcomes {
    #[outcome(message("Cognition {}\n  Agent: {}\n  Texture: {}\n  Content: {}\n  Created: {}", .0.id, .0.agent_id, .0.texture, .0.content, .0.created_at))]
    CognitionDetails(Cognition),
}
