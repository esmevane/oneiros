use oneiros_model::Texture;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListTexturesOutcomes {
    #[outcome(message("No textures configured."))]
    NoTextures,

    #[outcome(message("Textures: {0:?}"))]
    Textures(Vec<Texture>),
}
