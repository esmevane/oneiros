use oneiros_model::TextureRecord;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowTextureOutcomes {
    #[outcome(message("Texture '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    TextureDetails(TextureRecord),
}
