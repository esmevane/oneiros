use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Reflection {
    pub agent: AgentRecord,
    #[serde(skip)]
    pub prompt: String,
}
