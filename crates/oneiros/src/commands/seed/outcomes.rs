use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SeedOutcomes {
    #[outcome(transparent)]
    Texture(#[from] SetTextureOutcomes),
    #[outcome(transparent)]
    Level(#[from] SetLevelOutcomes),
    #[outcome(transparent)]
    Persona(#[from] SetPersonaOutcomes),
    #[outcome(transparent)]
    Agent(#[from] CreateAgentOutcomes),
    #[outcome(transparent)]
    Sensation(#[from] SetSensationOutcomes),
    #[outcome(transparent)]
    Nature(#[from] SetNatureOutcomes),
    #[outcome(transparent)]
    Core(#[from] CoreSeedOutcomes),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CoreSeedOutcomes {
    #[outcome(message("Seed failed for {0} '{1}': {2}"), level = "warn")]
    SeedFailed(String, String, String),
    #[outcome(message("Core seed complete."))]
    SeedComplete,
}

impl CoreSeedOutcomes {
    pub fn failed(
        kind: impl AsRef<str>,
        name: impl AsRef<str>,
        error: impl core::error::Error,
    ) -> Self {
        Self::SeedFailed(
            kind.as_ref().to_string(),
            name.as_ref().to_string(),
            error.to_string(),
        )
    }
}
