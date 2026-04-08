use std::collections::HashSet;

use crate::*;

/// Pressure reducer — computes gauge values from in-memory canon state.
///
/// Runs last in the pipeline, so all domain collections (cognitions,
/// memories, experiences, connections, urges) are already up-to-date
/// for the current event. Cross-references them to produce pressure
/// readings without any SQLite queries.
pub struct PressureState;

impl PressureState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        // Track continuity timestamps before computing pressure
        Self::track_continuity(&mut canon, event);

        // Collect agent info we need before borrowing canon immutably
        let agents: Vec<(AgentId, AgentName)> = Self::resolve_agents(&canon, event)
            .into_iter()
            .map(|a| (a.id, a.name.clone()))
            .collect();

        if agents.is_empty() {
            return canon;
        }

        let now = Timestamp::now();
        let referenced = Self::referenced_ids(&canon);
        let urge_names: Vec<UrgeName> = canon.urges.values().map(|u| u.name.clone()).collect();

        // Compute all gauges, then apply them
        let mut updates: Vec<Pressure> = vec![];

        for (agent_id, _agent_name) in &agents {
            if let Some(agent) = canon.agents.get(*agent_id) {
                for urge_name in &urge_names {
                    let gauge = Self::compute_gauge(&canon, &referenced, urge_name, agent);

                    let id = canon
                        .pressures
                        .values()
                        .find(|p| p.agent_id == *agent_id && p.urge == *urge_name)
                        .map(|p| p.id)
                        .unwrap_or_else(PressureId::new);

                    updates.push(Pressure {
                        id,
                        agent_id: *agent_id,
                        urge: urge_name.clone(),
                        data: gauge,
                        updated_at: now,
                    });
                }
            }
        }

        for pressure in &updates {
            canon.pressures.set(pressure);
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }

    /// Update continuity timestamps when we see lifecycle events.
    fn track_continuity(canon: &mut BrainCanon, event: &Events) {
        match event {
            Events::Continuity(ContinuityEvents::Introspected(e)) => {
                canon
                    .continuity_timestamps
                    .get_or_default(&e.agent)
                    .last_introspect = Some(e.created_at);
            }
            Events::Continuity(ContinuityEvents::Reflected(e)) => {
                canon
                    .continuity_timestamps
                    .get_or_default(&e.agent)
                    .last_reflect = Some(e.created_at);
            }
            Events::Continuity(ContinuityEvents::Dreamed(e)) => {
                canon
                    .continuity_timestamps
                    .get_or_default(&e.agent)
                    .last_dream = Some(e.created_at);
            }
            Events::Continuity(ContinuityEvents::Slept(e)) => {
                canon
                    .continuity_timestamps
                    .get_or_default(&e.agent)
                    .last_sleep = Some(e.created_at);
            }
            _ => {}
        }
    }

    /// Determine which agents need pressure recomputed for this event.
    fn resolve_agents<'a>(canon: &'a BrainCanon, event: &Events) -> Vec<&'a Agent> {
        match event {
            Events::Cognition(CognitionEvents::CognitionAdded(c)) => {
                canon.agents.get(c.agent_id).into_iter().collect()
            }
            Events::Memory(MemoryEvents::MemoryAdded(m)) => {
                canon.agents.get(m.agent_id).into_iter().collect()
            }
            Events::Experience(ExperienceEvents::ExperienceCreated(e)) => {
                canon.agents.get(e.agent_id).into_iter().collect()
            }
            Events::Continuity(ContinuityEvents::Introspected(e)) => {
                canon.agents.find_by_name(&e.agent).into_iter().collect()
            }
            Events::Continuity(ContinuityEvents::Reflected(e)) => {
                canon.agents.find_by_name(&e.agent).into_iter().collect()
            }
            Events::Continuity(ContinuityEvents::Dreamed(e)) => {
                canon.agents.find_by_name(&e.agent).into_iter().collect()
            }
            Events::Continuity(ContinuityEvents::Slept(e)) => {
                canon.agents.find_by_name(&e.agent).into_iter().collect()
            }
            Events::Continuity(ContinuityEvents::Sensed(e)) => {
                canon.agents.find_by_name(&e.agent).into_iter().collect()
            }
            // Connection events may affect orphaned/unconnected counts for any agent
            Events::Connection(_) => canon.agents.values().collect(),
            _ => vec![],
        }
    }

    /// Build a set of all entity IDs referenced by any connection.
    fn referenced_ids(canon: &BrainCanon) -> HashSet<String> {
        let mut ids = HashSet::new();
        for conn in canon.connections.values() {
            Self::collect_ref_id(&conn.from_ref, &mut ids);
            Self::collect_ref_id(&conn.to_ref, &mut ids);
        }
        ids
    }

    fn collect_ref_id(entity_ref: &Ref, ids: &mut HashSet<String>) {
        let Ref::V0(resource) = entity_ref;
        let id = match resource {
            Resource::Cognition(id) => id.to_string(),
            Resource::Experience(id) => id.to_string(),
            Resource::Memory(id) => id.to_string(),
            Resource::Agent(id) => id.to_string(),
            Resource::Connection(id) => id.to_string(),
            _ => return,
        };
        ids.insert(id);
    }

    fn compute_gauge(
        canon: &BrainCanon,
        referenced: &HashSet<String>,
        urge: &UrgeName,
        agent: &Agent,
    ) -> Gauge {
        match urge.as_str() {
            "introspect" => {
                Gauge::Introspect(Self::compute_introspect(canon, &agent.id, &agent.name))
            }
            "catharsis" => Gauge::Catharsis(Self::compute_catharsis(
                canon,
                referenced,
                &agent.id,
                &agent.name,
            )),
            "recollect" => Gauge::Recollect(Self::compute_recollect(canon, referenced, &agent.id)),
            "retrospect" => Gauge::Retrospect(Self::compute_retrospect(canon, &agent.id)),
            _ => Gauge::Introspect(IntrospectGauge::from_inputs(IntrospectInputs {
                hours_since_last_introspect: 0.0,
                total_cognitions: 0,
                working_cognitions: 0,
                cognitions_since_introspect: 0,
                memories_since_introspect: 0,
                session_cognition_count: 0,
            })),
        }
    }

    fn compute_introspect(
        canon: &BrainCanon,
        agent_id: &AgentId,
        agent_name: &AgentName,
    ) -> IntrospectGauge {
        let agent_cognitions: Vec<&Cognition> = canon
            .cognitions
            .values()
            .filter(|c| c.agent_id == *agent_id)
            .collect();

        let total_cognitions = agent_cognitions.len() as u64;
        let working_cognitions = agent_cognitions
            .iter()
            .filter(|c| c.texture.as_str() == "working")
            .count() as u64;

        let timestamps = canon.continuity_timestamps.get(agent_name);
        let last_introspect_at = timestamps.and_then(|t| t.last_introspect);
        let last_dream_at = timestamps.and_then(|t| t.last_dream);

        let hours_since = Self::hours_since(last_introspect_at.as_ref());

        // Cognitions since last introspect
        let cognitions_since = match last_introspect_at {
            Some(t) => agent_cognitions.iter().filter(|c| c.created_at > t).count() as u64,
            None => total_cognitions,
        };

        // Memories since last introspect
        let memories_since = match last_introspect_at {
            Some(t) => canon
                .memories
                .values()
                .filter(|m| m.agent_id == *agent_id && m.created_at > t)
                .count() as u64,
            None => canon
                .memories
                .values()
                .filter(|m| m.agent_id == *agent_id)
                .count() as u64,
        };

        // Session cognitions (since last wake/dream)
        let session_cognition_count = match last_dream_at {
            Some(t) => agent_cognitions.iter().filter(|c| c.created_at > t).count() as u64,
            None => total_cognitions,
        };

        IntrospectGauge::from_inputs(IntrospectInputs {
            hours_since_last_introspect: hours_since,
            total_cognitions,
            working_cognitions,
            cognitions_since_introspect: cognitions_since,
            memories_since_introspect: memories_since,
            session_cognition_count,
        })
    }

    fn compute_catharsis(
        canon: &BrainCanon,
        referenced: &HashSet<String>,
        agent_id: &AgentId,
        agent_name: &AgentName,
    ) -> CatharsisGauge {
        let agent_cognitions: Vec<&Cognition> = canon
            .cognitions
            .values()
            .filter(|c| c.agent_id == *agent_id)
            .collect();

        let total_cognitions = agent_cognitions.len() as u64;
        let working_cognitions = agent_cognitions
            .iter()
            .filter(|c| c.texture.as_str() == "working")
            .count() as u64;

        let tensions_count = canon
            .experiences
            .values()
            .filter(|e| e.agent_id == *agent_id && e.sensation.as_str() == "tensions")
            .count() as u64;

        let hours_since_reflect = canon
            .continuity_timestamps
            .get(agent_name)
            .and_then(|t| t.last_reflect)
            .map(|t| Self::hours_since(Some(&t)))
            .unwrap_or(24.0);

        // Orphaned cognitions: not referenced by any connection
        let orphaned = agent_cognitions
            .iter()
            .filter(|c| !referenced.contains(&c.id.to_string()))
            .count() as u64;

        CatharsisGauge::from_inputs(CatharsisInputs {
            tensions_experience_count: tensions_count,
            total_cognitions,
            working_cognitions,
            hours_since_last_reflect: hours_since_reflect,
            orphaned_cognitions: orphaned,
        })
    }

    fn compute_recollect(
        canon: &BrainCanon,
        referenced: &HashSet<String>,
        agent_id: &AgentId,
    ) -> RecollectGauge {
        let agent_memories: Vec<&Memory> = canon
            .memories
            .values()
            .filter(|m| m.agent_id == *agent_id)
            .collect();

        let session_memories = agent_memories
            .iter()
            .filter(|m| m.level.as_str() == "session")
            .count() as u64;

        let working_memories = agent_memories
            .iter()
            .filter(|m| m.level.as_str() == "working")
            .count() as u64;

        let agent_experiences: Vec<&Experience> = canon
            .experiences
            .values()
            .filter(|e| e.agent_id == *agent_id)
            .collect();

        let total_experiences = agent_experiences.len() as u64;
        let connected_experiences = agent_experiences
            .iter()
            .filter(|e| referenced.contains(&e.id.to_string()))
            .count() as u64;
        let unconnected = total_experiences - connected_experiences.min(total_experiences);

        // Hours since last memory added
        let last_memory_at = agent_memories.iter().map(|m| &m.created_at).max().copied();
        let hours_since_memory = Self::hours_since(last_memory_at.as_ref());

        RecollectGauge::from_inputs(RecollectInputs {
            session_memory_count: session_memories,
            total_experiences,
            unconnected_experiences: unconnected,
            hours_since_last_memory: hours_since_memory,
            working_memory_count: working_memories,
        })
    }

    fn compute_retrospect(canon: &BrainCanon, agent_id: &AgentId) -> RetrospectGauge {
        let agent_memories: Vec<&Memory> = canon
            .memories
            .values()
            .filter(|m| m.agent_id == *agent_id)
            .collect();

        let last_archival_at = agent_memories
            .iter()
            .filter(|m| m.level.as_str() == "archival")
            .map(|m| &m.created_at)
            .max()
            .copied();

        let last_project_at = agent_memories
            .iter()
            .filter(|m| m.level.as_str() == "project")
            .map(|m| &m.created_at)
            .max()
            .copied();

        let total_experiences = canon
            .experiences
            .values()
            .filter(|e| e.agent_id == *agent_id)
            .count() as u64;

        RetrospectGauge::from_inputs(RetrospectInputs {
            hours_since_last_archival: Self::hours_since(last_archival_at.as_ref()),
            hours_since_last_project_memory: Self::hours_since(last_project_at.as_ref()),
            sessions_since_retrospect: 0,
            total_experience_count: total_experiences,
        })
    }

    fn hours_since(timestamp: Option<&Timestamp>) -> f64 {
        match timestamp {
            Some(t) => {
                if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&t.as_string()) {
                    let elapsed = chrono::Utc::now().signed_duration_since(parsed);
                    (elapsed.num_seconds() as f64 / 3600.0).max(0.0)
                } else {
                    24.0
                }
            }
            None => 24.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded_canon() -> BrainCanon {
        let mut canon = BrainCanon::default();

        let agent = Agent::builder()
            .name("gov.process")
            .persona("process")
            .description("Governor")
            .prompt("You govern")
            .build();

        let urge_introspect = Urge::builder()
            .name("introspect")
            .description("Look inward")
            .prompt("")
            .build();

        let urge_catharsis = Urge::builder()
            .name("catharsis")
            .description("Release tension")
            .prompt("")
            .build();

        canon.agents.set(&agent);
        canon.urges.set(&urge_introspect);
        canon.urges.set(&urge_catharsis);

        canon
    }

    #[test]
    fn ignores_irrelevant_events() {
        let canon = seeded_canon();
        let event = Events::Level(LevelEvents::LevelSet(
            Level::builder()
                .name("working")
                .description("Short-term")
                .prompt("")
                .build(),
        ));

        let next = PressureState::reduce(canon, &event);
        assert!(next.pressures.is_empty());
    }

    #[test]
    fn computes_on_cognition_added() {
        let mut canon = seeded_canon();
        let agent = canon
            .agents
            .find_by_name(&AgentName::new("gov.process"))
            .unwrap();
        let agent_id = agent.id;

        let cognition = Cognition::builder()
            .agent_id(agent_id)
            .texture("observation")
            .content("A thought")
            .build();

        // Simulate the full pipeline: cognition reducer then pressure reducer
        let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition));
        canon = CognitionState::reduce(canon, &event);
        let next = PressureState::reduce(canon, &event);

        // Should have 2 pressure entries (one per urge)
        assert_eq!(next.pressures.len(), 2);
    }

    #[test]
    fn counts_working_cognitions() {
        let mut canon = seeded_canon();
        let agent = canon
            .agents
            .find_by_name(&AgentName::new("gov.process"))
            .unwrap();
        let agent_id = agent.id;

        let working = Cognition::builder()
            .agent_id(agent_id)
            .texture("working")
            .content("Working thought")
            .build();
        let observation = Cognition::builder()
            .agent_id(agent_id)
            .texture("observation")
            .content("An observation")
            .build();

        canon.cognitions.set(&working);
        canon.cognitions.set(&observation);

        let event = Events::Cognition(CognitionEvents::CognitionAdded(observation));
        let next = PressureState::reduce(canon, &event);

        let introspect = next
            .pressures
            .values()
            .find(|p| p.urge.as_str() == "introspect")
            .expect("should have introspect pressure");

        if let Gauge::Introspect(g) = &introspect.data {
            assert_eq!(g.inputs.total_cognitions, 2);
            assert_eq!(g.inputs.working_cognitions, 1);
        } else {
            panic!("expected introspect gauge");
        }
    }

    #[test]
    fn orphaned_cognitions_decrease_with_connections() {
        let mut canon = seeded_canon();
        let agent = canon
            .agents
            .find_by_name(&AgentName::new("gov.process"))
            .unwrap();
        let agent_id = agent.id;

        let cognition = Cognition::builder()
            .agent_id(agent_id)
            .texture("observation")
            .content("Orphaned thought")
            .build();
        let cog_id = cognition.id;
        canon.cognitions.set(&cognition);

        // Before connection: cognition is orphaned
        let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition));
        let after_cog = PressureState::reduce(canon, &event);

        let catharsis_before = after_cog
            .pressures
            .values()
            .find(|p| p.urge.as_str() == "catharsis")
            .expect("catharsis");
        let orphaned_before = match &catharsis_before.data {
            Gauge::Catharsis(g) => g.inputs.orphaned_cognitions,
            _ => panic!("expected catharsis"),
        };
        assert_eq!(orphaned_before, 1);

        // Add a connection referencing the cognition
        let mut canon = after_cog;
        let connection = Connection::builder()
            .from_ref(Ref::cognition(cog_id))
            .to_ref(Ref::experience(ExperienceId::new()))
            .nature("context")
            .build();
        canon.connections.set(&connection);

        let conn_event = Events::Connection(ConnectionEvents::ConnectionCreated(connection));
        let after_conn = PressureState::reduce(canon, &conn_event);

        let catharsis_after = after_conn
            .pressures
            .values()
            .find(|p| p.urge.as_str() == "catharsis")
            .expect("catharsis");
        let orphaned_after = match &catharsis_after.data {
            Gauge::Catharsis(g) => g.inputs.orphaned_cognitions,
            _ => panic!("expected catharsis"),
        };
        assert_eq!(orphaned_after, 0);
    }

    #[test]
    fn tracks_continuity_timestamps() {
        let mut canon = seeded_canon();

        let event = Events::Continuity(ContinuityEvents::Introspected(ContinuityEvent {
            agent: AgentName::new("gov.process"),
            created_at: Timestamp::now(),
        }));

        canon = PressureState::reduce(canon, &event);

        let timestamps = canon
            .continuity_timestamps
            .get(&AgentName::new("gov.process"))
            .expect("should have timestamps");
        assert!(timestamps.last_introspect.is_some());
    }
}
