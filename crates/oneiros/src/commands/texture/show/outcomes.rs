use oneiros_model::Texture;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ShowTextureOutcomes {
    #[outcome(message("Texture '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    TextureDetails(Texture),
}
