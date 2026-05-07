use crate::*;

#[derive(Clone, serde::Serialize)]
pub(crate) struct Introspection {
    pub(crate) agent: Agent,
    #[serde(skip)]
    pub(crate) prompt: String,
}
