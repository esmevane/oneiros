use std::collections::{HashSet, VecDeque};

use oneiros_db::Database;
use oneiros_model::*;
use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct DreamParams {
    pub recent_window: Option<usize>,
    pub dream_depth: Option<usize>,
    pub cognition_size: Option<usize>,
    pub recollection_level: Option<LevelName>,
    pub recollection_size: Option<usize>,
    pub experience_size: Option<usize>,
}

impl From<DreamParams> for DreamConfig {
    fn from(params: DreamParams) -> Self {
        let mut cfg = DreamConfig::default();
        if let Some(v) = params.recent_window {
            cfg.recent_window = v;
        }
        if let Some(v) = params.dream_depth {
            cfg.dream_depth = Some(v);
        }
        if let Some(v) = params.cognition_size {
            cfg.cognition_size = Some(v);
        }
        if let Some(v) = params.recollection_level {
            cfg.recollection_level = Some(v);
        }
        if let Some(v) = params.recollection_size {
            cfg.recollection_size = Some(v);
        }
        if let Some(v) = params.experience_size {
            cfg.experience_size = Some(v);
        }
        cfg
    }
}

pub(crate) struct DreamConfig {
    /// Number of recent cognitions and experiences to include
    /// in the orientation window.
    pub recent_window: usize,
    /// Maximum BFS traversal depth from the seed set.
    /// None means unlimited.
    pub dream_depth: Option<usize>,
    /// Maximum number of cognitions in the dream.
    /// None means unlimited.
    pub cognition_size: Option<usize>,
    /// Minimum memory level to include (log-level semantics).
    /// Core memories are always included regardless of this setting.
    /// None means include all levels.
    pub recollection_level: Option<LevelName>,
    /// Maximum number of non-core memories in the dream.
    /// None means unlimited.
    pub recollection_size: Option<usize>,
    /// Maximum number of experiences in the dream.
    /// None means unlimited.
    pub experience_size: Option<usize>,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self {
            recent_window: 5,
            dream_depth: Some(1),
            cognition_size: Some(20),
            recollection_level: Some(LevelName::new("project")),
            recollection_size: Some(30),
            experience_size: Some(10),
        }
    }
}

fn level_priority(name: &LevelName) -> usize {
    match name.as_ref() {
        "core" => 5,
        "working" => 4,
        "session" => 3,
        "project" => 2,
        "archival" => 1,
        _ => 0,
    }
}

pub(crate) struct DreamCollector<'a> {
    db: &'a Database,
    config: DreamConfig,
}

impl<'a> DreamCollector<'a> {
    pub fn new(db: &'a Database, config: DreamConfig) -> Self {
        Self { db, config }
    }

    pub fn collect(&self, agent: &Agent) -> Result<DreamContext, Error> {
        // FOUNDATION — always included
        let persona = self
            .db
            .get_persona(&agent.persona)?
            .ok_or(crate::NotFound::Persona(agent.persona.clone()))?;

        // Vocabulary types — system-wide
        let textures = self.db.list_textures()?;
        let levels = self.db.list_levels()?;
        let sensations = self.db.list_sensations()?;
        let natures = self.db.list_natures()?;

        // MEMORY FILTERING — core always included, rest filtered by level + capped
        let all_memories = self.db.list_memories_by_agent(agent.id.to_string())?;
        let (core_memories, rest_memories): (Vec<_>, Vec<_>) = all_memories
            .into_iter()
            .partition(|m| m.level.as_ref() == "core");

        let filtered_rest = self.filter_memories(rest_memories);
        let mut memories: Vec<Memory> = core_memories;
        memories.extend(filtered_rest);
        memories.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        // Recent experiences provide orientation
        let recent_experiences = self
            .db
            .list_recent_experiences_by_agent(&agent.id, self.config.recent_window)?;

        // BFS seed: filtered memories + recent experiences (not all memories)
        let seed_refs: Vec<Ref> = memories
            .iter()
            .map(|m| Ref::memory(m.id))
            .chain(recent_experiences.iter().map(|e| Ref::experience(e.id)))
            .collect();

        // GRAPH TRAVERSAL — BFS from foundation through all connection types.
        // Agent scoping is topological: the BFS starts from this agent's
        // entities, so it only discovers things connected to them.
        let mut visited: HashSet<Ref> = HashSet::new();
        let mut frontier: VecDeque<(Ref, usize)> = seed_refs.into_iter().map(|r| (r, 0)).collect();

        let mut cognition_ids: HashSet<CognitionId> = HashSet::new();
        let mut experience_ids: HashSet<ExperienceId> =
            recent_experiences.iter().map(|e| e.id).collect();
        let mut connections: Vec<Connection> = Vec::new();
        let mut seen_conn_ids: HashSet<ConnectionId> = HashSet::new();

        while let Some((current, depth)) = frontier.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }

            let node_connections = self.db.list_connections_by_ref(&current)?;

            for conn in node_connections {
                if !seen_conn_ids.insert(conn.id) {
                    continue;
                }

                let other = if conn.from_ref == current {
                    &conn.to_ref
                } else {
                    &conn.from_ref
                };

                // Only discover entities within the depth limit.
                let neighbor_depth = depth + 1;
                let within_depth = self
                    .config
                    .dream_depth
                    .is_none_or(|max| neighbor_depth <= max);

                if within_depth {
                    match other.resource() {
                        Resource::Cognition(id) => {
                            cognition_ids.insert(*id);
                        }
                        Resource::Experience(id) => {
                            experience_ids.insert(*id);
                        }
                        _ => {}
                    }

                    if !visited.contains(other) {
                        frontier.push_back((other.clone(), neighbor_depth));
                    }
                }

                connections.push(conn);
            }
        }

        // Decide cognition selection strategy
        let mut cognitions = if connections.is_empty() {
            // FTS5 FALLBACK — no connections means sparse graph, include ALL cognitions
            self.db.list_cognitions_by_agent(agent.id.to_string())?
        } else {
            self.assemble_cognitions(agent, &cognition_ids)?
        };

        // Apply cognition_size cap — keep the most recent.
        if let Some(max) = self.config.cognition_size
            && cognitions.len() > max
        {
            cognitions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            cognitions = cognitions.split_off(cognitions.len() - max);
        }

        // Assemble experiences: recent + graph-discovered, deduped
        let mut experiences = self.assemble_experiences(&recent_experiences, &experience_ids)?;

        // Apply experience_size cap — keep the most recent.
        if let Some(max) = self.config.experience_size
            && experiences.len() > max
        {
            experiences.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            experiences = experiences.split_off(experiences.len() - max);
        }

        // Filter connections: only retain edges whose endpoints are in the result.
        let included_refs: HashSet<Ref> = memories
            .iter()
            .map(|m| Ref::memory(m.id))
            .chain(cognitions.iter().map(|c| Ref::cognition(c.id)))
            .chain(experiences.iter().map(|e| Ref::experience(e.id)))
            .collect();

        let connections = connections
            .into_iter()
            .filter(|conn| {
                included_refs.contains(&conn.from_ref) && included_refs.contains(&conn.to_ref)
            })
            .collect();

        Ok(DreamContext {
            agent: agent.clone(),
            persona,
            memories,
            cognitions,
            experiences,
            connections,
            textures,
            levels,
            sensations,
            natures,
        })
    }

    fn filter_memories(&self, mut memories: Vec<Memory>) -> Vec<Memory> {
        // Filter by level threshold (log-level semantics)
        if let Some(threshold) = &self.config.recollection_level {
            let min_priority = level_priority(threshold);
            memories.retain(|m| level_priority(&m.level) >= min_priority);
        }

        // Sort by created_at descending for recency-based capping
        memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Cap at recollection_size
        if let Some(max) = self.config.recollection_size {
            memories.truncate(max);
        }

        memories
    }

    fn assemble_cognitions(
        &self,
        agent: &Agent,
        graph_ids: &HashSet<CognitionId>,
    ) -> Result<Vec<Cognition>, Error> {
        // ORIENTATION — recent cognitions for continuity
        let recent = self
            .db
            .list_recent_cognitions_by_agent(&agent.id, self.config.recent_window)?;

        let mut cognitions: Vec<Cognition> = Vec::new();
        let mut seen: HashSet<CognitionId> = HashSet::new();

        // Recent first (already fetched)
        for cog in recent {
            if seen.insert(cog.id) {
                cognitions.push(cog);
            }
        }

        // Graph-discovered
        for id in graph_ids {
            if seen.insert(*id)
                && let Some(cog) = self.db.get_cognition(id.to_string())?
            {
                cognitions.push(cog);
            }
        }

        cognitions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(cognitions)
    }

    fn assemble_experiences(
        &self,
        recent: &[Experience],
        graph_ids: &HashSet<ExperienceId>,
    ) -> Result<Vec<Experience>, Error> {
        let mut experiences: Vec<Experience> = Vec::new();
        let mut seen: HashSet<ExperienceId> = HashSet::new();

        // Recent first (already fetched)
        for exp in recent {
            if seen.insert(exp.id) {
                experiences.push(exp.clone());
            }
        }

        // Graph-discovered
        for id in graph_ids {
            if seen.insert(*id)
                && let Some(exp) = self.db.get_experience(id.to_string())?
            {
                experiences.push(exp);
            }
        }

        experiences.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(experiences)
    }
}
