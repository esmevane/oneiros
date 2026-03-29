use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Introspection {
    pub agent: Agent,
    #[serde(skip)]
    pub prompt: String,
}
