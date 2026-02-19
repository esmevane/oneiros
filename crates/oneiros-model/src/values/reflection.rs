use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Reflection {
    pub agent: Identity<AgentId, Agent>,
    #[serde(skip)]
    pub prompt: String,
}
