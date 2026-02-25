use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Observation {
    pub agent: Agent,
    #[serde(skip)]
    pub prompt: String,
}
