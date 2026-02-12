use oneiros_model::TextureName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum RemoveTextureOutcomes {
    #[outcome(message("Texture '{0}' removed."))]
    TextureRemoved(TextureName),
}
