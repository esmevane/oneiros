use oneiros_model::TextureRecord;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListTexturesOutcomes {
    #[outcome(message("No textures configured."))]
    NoTextures,

    #[outcome(message("Textures: {0:?}"))]
    Textures(Vec<TextureRecord>),
}
