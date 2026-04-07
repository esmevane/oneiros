use crate::*;

/// The full state of a brain database — all domain entities for one project.
///
/// Reducers fold events into this struct as a pure function.
/// Aggregating reducers (Pressure) can read across all tables
/// because they receive the full canon.
#[derive(Default, Clone)]
pub struct BrainCanon {
    pub agents: Agents,
    pub cognitions: Cognitions,
    pub memories: Memories,
    pub experiences: Experiences,
    pub connections: Connections,
    pub levels: Levels,
    pub textures: Textures,
    pub sensations: Sensations,
    pub natures: Natures,
    pub personas: Personas,
    pub urges: Urges,
    pub storage: StorageEntries,
    pub pressures: Pressures,
}
