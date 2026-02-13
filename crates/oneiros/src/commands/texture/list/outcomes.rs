use oneiros_model::Texture;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListTexturesOutcomes {
    #[outcome(message("No textures configured."))]
    NoTextures,

    #[outcome(message("Textures: {0:?}"))]
    Textures(Vec<Texture>),
}
