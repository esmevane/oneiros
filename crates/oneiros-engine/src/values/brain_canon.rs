use std::collections::HashMap;

use crate::*;

/// The full state of a brain database — all domain entities for one project.
///
/// Reducers fold events into this struct as a pure function.
/// Aggregating reducers (Pressure) can read across all tables
/// because they receive the full canon.
#[derive(Default, Clone)]
pub struct BrainCanon {
    pub agents: HashMap<String, Agent>,
    pub cognitions: HashMap<String, Cognition>,
    pub memories: HashMap<String, Memory>,
    pub experiences: HashMap<String, Experience>,
    pub connections: HashMap<String, Connection>,
    pub levels: HashMap<String, Level>,
    pub textures: HashMap<String, Texture>,
    pub sensations: HashMap<String, Sensation>,
    pub natures: HashMap<String, Nature>,
    pub personas: HashMap<String, Persona>,
    pub urges: HashMap<String, Urge>,
    pub storage: HashMap<String, StorageEntry>,
    pub pressures: HashMap<String, Pressure>,
}
