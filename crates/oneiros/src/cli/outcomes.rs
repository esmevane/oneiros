use crate::*;
use oneiros_outcomes::Outcome;

#[derive(serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum CliOutcomes {
    #[outcome(transparent)]
    Agent(#[from] AgentOutcomes),
    #[outcome(transparent)]
    Cognition(#[from] CognitionOutcomes),
    #[outcome(transparent)]
    Doctor(#[from] DoctorOutcomes),
    #[outcome(transparent)]
    Dream(#[from] DreamOutcomes),
    #[outcome(transparent)]
    Experience(#[from] ExperienceOutcomes),
    #[outcome(transparent)]
    Sensation(#[from] SensationOutcomes),
    #[outcome(transparent)]
    Guidebook(#[from] GuidebookOutcomes),
    #[outcome(transparent)]
    Introspect(#[from] IntrospectOutcomes),
    #[outcome(transparent)]
    Level(#[from] LevelOutcomes),
    #[outcome(transparent)]
    Memory(#[from] MemoryOutcomes),
    #[outcome(transparent)]
    Persona(#[from] PersonaOutcomes),
    #[outcome(transparent)]
    Reflect(#[from] ReflectOutcomes),
    #[outcome(transparent)]
    Seed(#[from] SeedOutcomes),
    #[outcome(transparent)]
    Skill(#[from] SkillOutcomes),
    #[outcome(transparent)]
    Storage(#[from] StorageOutcomes),
    #[outcome(transparent)]
    Project(#[from] ProjectOutcomes),
    #[outcome(transparent)]
    Service(#[from] ServiceOutcomes),
    #[outcome(transparent)]
    System(#[from] SystemOutcomes),
    #[outcome(transparent)]
    Texture(#[from] TextureOutcomes),
}
