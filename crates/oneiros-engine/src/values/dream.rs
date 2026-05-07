use crate::*;

#[derive(Clone, serde::Serialize)]
pub(crate) struct Dream {
    #[serde(flatten)]
    pub(crate) context: DreamContext,
    #[serde(skip)]
    pub(crate) prompt: String,
}
