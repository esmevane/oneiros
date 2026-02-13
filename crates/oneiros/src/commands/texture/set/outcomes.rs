use oneiros_model::TextureName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetTextureOutcomes {
    #[outcome(message("Texture '{0}' set."))]
    TextureSet(TextureName),
}
