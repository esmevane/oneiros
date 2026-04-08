use std::collections::HashMap;

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
    /// Per-agent continuity timestamps for pressure computation.
    /// Not CRDT-synced — derived from events during replay.
    pub continuity_timestamps: ContinuityTimestamps,
}

/// Tracks the most recent continuity event timestamps per agent.
/// Used by the pressure reducer to compute "hours since" factors.
#[derive(Default, Clone)]
pub struct ContinuityTimestamps {
    /// Keyed by agent name string → per-agent timestamps.
    agents: HashMap<String, AgentTimestamps>,
}

/// Per-agent timestamps for the most recent continuity events.
#[derive(Default, Clone)]
pub struct AgentTimestamps {
    pub last_introspect: Option<Timestamp>,
    pub last_reflect: Option<Timestamp>,
    pub last_dream: Option<Timestamp>,
    pub last_sleep: Option<Timestamp>,
}

impl ContinuityTimestamps {
    pub fn get(&self, agent: &AgentName) -> Option<&AgentTimestamps> {
        self.agents.get(agent.as_str())
    }

    pub fn get_or_default(&mut self, agent: &AgentName) -> &mut AgentTimestamps {
        self.agents.entry(agent.as_str().to_string()).or_default()
    }
}
