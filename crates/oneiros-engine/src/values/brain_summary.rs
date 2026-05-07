use serde::Serialize;

use crate::*;

/// Brain summary — counts and recent data for the dashboard.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct BrainSummary {
    pub(crate) agents: Vec<Agent>,
    pub(crate) agent_count: usize,
    pub(crate) cognition_count: usize,
    pub(crate) memory_count: usize,
    pub(crate) experience_count: usize,
    pub(crate) connection_count: usize,
    pub(crate) event_count: usize,
    pub(crate) recent_cognitions: Vec<Cognition>,
}
