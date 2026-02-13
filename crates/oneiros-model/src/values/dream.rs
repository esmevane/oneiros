use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct Dream {
    #[serde(flatten)]
    pub context: DreamContext,
    #[serde(skip)]
    pub prompt: String,
}
