use serde::Serialize;

use crate::*;

/// Brain summary — counts and recent data for the dashboard.
#[derive(Debug, Clone, Serialize)]
pub struct BrainSummary {
    pub agents: Vec<Agent>,
    pub agent_count: usize,
    pub cognition_count: usize,
    pub memory_count: usize,
    pub experience_count: usize,
    pub connection_count: usize,
    pub event_count: usize,
    pub recent_cognitions: Vec<Cognition>,
}
