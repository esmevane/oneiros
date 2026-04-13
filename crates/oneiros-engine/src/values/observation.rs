use crate::*;

#[derive(Clone, serde::Serialize)]
pub(crate) struct Observation {
    pub(crate) agent: Agent,
    #[serde(skip)]
    pub(crate) prompt: String,
}
