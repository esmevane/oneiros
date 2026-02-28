use std::collections::{HashSet, VecDeque};

use oneiros_db::Database;
use oneiros_model::*;

use crate::Error;

pub(crate) struct DreamConfig {
    /// Number of recent cognitions, memories, and experiences to include
    /// in the orientation window.
    pub recent_window: usize,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self { recent_window: 20 }
    }
}

pub(crate) struct DreamCollector<'a> {
    db: &'a Database,
    config: DreamConfig,
}

impl<'a> DreamCollector<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self {
            db,
            config: DreamConfig::default(),
        }
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

        // All memories form the skeleton (identity, learnings, history)
        let memories = self.db.list_memories_by_agent(agent.id.to_string())?;

        // Recent experiences provide orientation
        let recent_experiences = self
            .db
            .list_recent_experiences_by_agent(&agent.id, self.config.recent_window)?;

        // BFS seed: all memories + recent experiences
        let seed_refs: Vec<Ref> = memories
            .iter()
            .map(|m| Ref::memory(m.id))
            .chain(recent_experiences.iter().map(|e| Ref::experience(e.id)))
            .collect();

        // GRAPH TRAVERSAL — BFS from foundation through all connection types.
        // Agent scoping is topological: the BFS starts from this agent's
        // entities, so it only discovers things connected to them.
        let mut visited: HashSet<Ref> = HashSet::new();
        let mut frontier: VecDeque<Ref> = seed_refs.into_iter().collect();

        let mut cognition_ids: HashSet<CognitionId> = HashSet::new();
        let mut experience_ids: HashSet<ExperienceId> =
            recent_experiences.iter().map(|e| e.id).collect();
        let mut connections: Vec<Connection> = Vec::new();
        let mut seen_conn_ids: HashSet<ConnectionId> = HashSet::new();

        while let Some(current) = frontier.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }

            // Query connections touching this specific ref
            let node_connections = self.db.list_connections_by_ref(&current)?;

            for conn in node_connections {
                // Dedup connections (discovered from both endpoints)
                if !seen_conn_ids.insert(conn.id) {
                    continue;
                }

                // Determine the other end of the edge
                let other = if conn.from_ref == current {
                    &conn.to_ref
                } else {
                    &conn.from_ref
                };

                // Classify and collect the discovered entity
                match other.resource() {
                    Resource::Cognition(id) => {
                        cognition_ids.insert(*id);
                    }
                    Resource::Experience(id) => {
                        experience_ids.insert(*id);
                    }
                    _ => {}
                }

                // Push the other end for further traversal
                if !visited.contains(other) {
                    frontier.push_back(other.clone());
                }

                connections.push(conn);
            }
        }

        // Decide cognition selection strategy
        let cognitions = if connections.is_empty() {
            // FTS5 FALLBACK — no connections means sparse graph, include ALL cognitions
            self.db.list_cognitions_by_agent(agent.id.to_string())?
        } else {
            self.assemble_cognitions(agent, &cognition_ids)?
        };

        // Assemble experiences: recent + graph-discovered, deduped
        let experiences = self.assemble_experiences(&recent_experiences, &experience_ids)?;

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
