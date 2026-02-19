use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Introspection {
    pub agent: Identity<AgentId, Agent>,
    #[serde(skip)]
    pub prompt: String,
}
