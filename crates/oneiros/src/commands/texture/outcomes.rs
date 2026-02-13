use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum TextureOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetTextureOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveTextureOutcomes),
    #[outcome(transparent)]
    List(#[from] ListTexturesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowTextureOutcomes),
}
