use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Observation {
    pub agent: Identity<AgentId, Agent>,
    #[serde(skip)]
    pub prompt: String,
}
