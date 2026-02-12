use oneiros_model::TextureName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum SetTextureOutcomes {
    #[outcome(message("Texture '{0}' set."))]
    TextureSet(TextureName),
}
