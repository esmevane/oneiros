use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Observation {
    pub agent: AgentRecord,
    #[serde(skip)]
    pub prompt: String,
}
