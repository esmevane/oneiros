use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamContext {
    pub agent: AgentRecord,
    pub persona: PersonaRecord,
    pub memories: Vec<Record<MemoryId, Memory>>,
    pub cognitions: Vec<Record<CognitionId, Cognition>>,
    pub experiences: Vec<ExperienceRecord>,
    pub connections: Vec<Record<ConnectionId, Connection>>,
    pub textures: Vec<TextureRecord>,
    pub levels: Vec<LevelRecord>,
    pub sensations: Vec<SensationRecord>,
    pub natures: Vec<NatureRecord>,
}
