use oneiros_model::Memory;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowMemoryOutcomes {
    #[outcome(
        message("Memory {}\n  Agent: {}\n  Level: {}\n  Content: {}\n  Created: {}", .0.id, .0.agent_id, .0.level, .0.content, .0.created_at),
        prompt("Is this grounded in specific cognitions? Mark it with `oneiros experience create <agent> grounds <description>`.")
    )]
    MemoryDetails(Memory),
}
