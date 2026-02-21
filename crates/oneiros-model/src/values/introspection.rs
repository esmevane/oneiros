use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Introspection {
    pub agent: AgentRecord,
    #[serde(skip)]
    pub prompt: String,
}
