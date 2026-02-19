use serde::{Deserialize, Serialize};

use crate::{
    Agent, AgentId, Cognition, CognitionId, Experience, ExperienceId, Identity, Level, Memory,
    MemoryId, Persona, Sensation, Texture,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamContext {
    pub agent: Identity<AgentId, Agent>,
    pub persona: Persona,
    pub memories: Vec<Identity<MemoryId, Memory>>,
    pub cognitions: Vec<Identity<CognitionId, Cognition>>,
    pub experiences: Vec<Identity<ExperienceId, Experience>>,
    pub textures: Vec<Texture>,
    pub levels: Vec<Level>,
    pub sensations: Vec<Sensation>,
}
