use std::collections::HashMap;

use crate::*;

/// The full state of a brain database — all domain entities for one project.
///
/// Reducers fold events into this struct as a pure function.
/// Aggregating reducers (Pressure) can read across all tables
/// because they receive the full canon.
#[derive(Default, Clone)]
pub(crate) struct BrainCanon {
    pub(crate) agents: Agents,
    pub(crate) cognitions: Cognitions,
    pub(crate) memories: Memories,
    pub(crate) experiences: Experiences,
    pub(crate) connections: Connections,
    pub(crate) levels: Levels,
    pub(crate) textures: Textures,
    pub(crate) sensations: Sensations,
    pub(crate) natures: Natures,
    pub(crate) personas: Personas,
    pub(crate) urges: Urges,
    pub(crate) storage: StorageEntries,
    pub(crate) pressures: Pressures,
    /// Per-agent continuity timestamps for pressure computation.
    /// Not CRDT-synced — derived from events during replay.
    pub(crate) continuity_timestamps: ContinuityTimestamps,
}

/// Tracks the most recent continuity event timestamps per agent.
/// Used by the pressure reducer to compute "hours since" factors.
#[derive(Default, Clone)]
pub(crate) struct ContinuityTimestamps {
    /// Keyed by agent name string → per-agent timestamps.
    agents: HashMap<String, AgentTimestamps>,
}

/// Per-agent timestamps for the most recent continuity events.
#[derive(Default, Clone)]
pub(crate) struct AgentTimestamps {
    pub(crate) last_introspect: Option<Timestamp>,
    pub(crate) last_reflect: Option<Timestamp>,
    pub(crate) last_dream: Option<Timestamp>,
    pub(crate) last_sleep: Option<Timestamp>,
}

impl ContinuityTimestamps {
    pub(crate) fn get(&self, agent: &AgentName) -> Option<&AgentTimestamps> {
        self.agents.get(agent.as_str())
    }

    pub(crate) fn get_or_default(&mut self, agent: &AgentName) -> &mut AgentTimestamps {
        self.agents.entry(agent.as_str().to_string()).or_default()
    }
}
