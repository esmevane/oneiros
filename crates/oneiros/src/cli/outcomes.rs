use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Outcome)]
pub enum CliOutcomes {
    #[outcome(transparent)]
    Agent(#[from] AgentOutcomes),
    #[outcome(transparent)]
    Cognition(#[from] CognitionOutcomes),
    #[outcome(transparent)]
    Doctor(#[from] DoctorOutcomes),
    #[outcome(transparent)]
    Level(#[from] LevelOutcomes),
    #[outcome(transparent)]
    Persona(#[from] PersonaOutcomes),
    #[outcome(transparent)]
    Project(#[from] ProjectOutcomes),
    #[outcome(transparent)]
    Service(#[from] ServiceOutcomes),
    #[outcome(transparent)]
    System(#[from] SystemOutcomes),
    #[outcome(transparent)]
    Texture(#[from] TextureOutcomes),
}
