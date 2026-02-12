use oneiros_model::Memory;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ShowMemoryOutcomes {
    #[outcome(message("Memory {}\n  Agent: {}\n  Level: {}\n  Content: {}\n  Created: {}", .0.id, .0.agent_id, .0.level, .0.content, .0.created_at))]
    MemoryDetails(Memory),
}
