use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum AgentOutcomes {
    #[outcome(transparent)]
    Create(#[from] CreateAgentOutcomes),
    #[outcome(transparent)]
    Update(#[from] UpdateAgentOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveAgentOutcomes),
    #[outcome(transparent)]
    List(#[from] ListAgentsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowAgentOutcomes),
}
