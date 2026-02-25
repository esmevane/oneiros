use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Reflection {
    pub agent: Agent,
    #[serde(skip)]
    pub prompt: String,
}
