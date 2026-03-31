//! Continuity service — composes other domain services into workflows.
//!
//! These operations don't have their own repos. They gather data from
//! multiple domains, perform the operation, and emit continuity events.

use std::collections::{HashSet, VecDeque};

use crate::*;

fn level_priority(name: &LevelName) -> usize {
    match name.as_str() {
        "core" => 5,
        "working" => 4,
        "session" => 3,
        "project" => 2,
        "archival" => 1,
        _ => 0,
    }
}

pub struct ContinuityService;

impl ContinuityService {
    /// Emerge — create an agent and immediately activate its continuity.
    pub async fn emerge(
        context: &ProjectContext,
        EmergeAgent {
            name,
            persona,
            description,
        }: &EmergeAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let created = AgentService::create(
            context,
            &CreateAgent::builder()
                .name(name.clone())
                .persona(persona.clone())
                .description(description.clone())
                .build(),
        )
        .await?;

        let agent_name = match created {
            AgentResponse::AgentCreated(n) => n,
            other => {
                return Err(ContinuityError::UnexpectedResponse(format!("{other:?}")));
            }
        };

        // Wake activates continuity; then gather the full context for the response.
        Self::wake(
            context,
            &WakeAgent::builder().agent(agent_name.clone()).build(),
            overrides,
        )
        .await?;
        let dream = Self::gather_context(context, &agent_name, overrides)?;
        Ok(ContinuityResponse::Emerged(dream))
    }

    /// Recede — retire an agent, ending its continuity.
    pub async fn recede(
        context: &ProjectContext,
        selector: &RecedeAgent,
    ) -> Result<ContinuityResponse, ContinuityError> {
        AgentService::remove(
            context,
            &RemoveAgent::builder().name(selector.agent.clone()).build(),
        )
        .await?;
        Ok(ContinuityResponse::Receded(selector.agent.clone()))
    }

    /// Status — read the current state of an agent's continuity.
    pub fn status(
        context: &ProjectContext,
        selector: &StatusAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;
        Ok(ContinuityResponse::Status(dream))
    }

    /// Wake — restore an agent's full cognitive context (initial session start).
    pub async fn wake(
        context: &ProjectContext,
        selector: &WakeAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;

        context
            .emit(ContinuityEvents::Dreamed(ContinuityEvent {
                agent: selector.agent.clone(),
                created_at: Timestamp::now(),
            }))
            .await?;

        Ok(ContinuityResponse::Waking(dream))
    }

    /// Dream — restore an agent's full cognitive context.
    pub async fn dream(
        context: &ProjectContext,
        selector: &DreamAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;

        context
            .emit(ContinuityEvents::Dreamed(ContinuityEvent {
                agent: selector.agent.clone(),
                created_at: Timestamp::now(),
            }))
            .await?;

        Ok(ContinuityResponse::Dreaming(dream))
    }

    /// Introspect — look inward, consolidate cognitive state.
    pub async fn introspect(
        context: &ProjectContext,
        selector: &IntrospectAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;

        context
            .emit(ContinuityEvents::Introspected(ContinuityEvent {
                agent: selector.agent.clone(),
                created_at: Timestamp::now(),
            }))
            .await?;

        Ok(ContinuityResponse::Introspecting(dream))
    }

    /// Reflect — pause on something significant.
    pub async fn reflect(
        context: &ProjectContext,
        selector: &ReflectAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;

        context
            .emit(ContinuityEvents::Reflected(ContinuityEvent {
                agent: selector.agent.clone(),
                created_at: Timestamp::now(),
            }))
            .await?;

        Ok(ContinuityResponse::Reflecting(dream))
    }

    /// Sense — receive and interpret something from outside.
    pub async fn sense(
        context: &ProjectContext,
        selector: &SenseContent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;

        context
            .emit(ContinuityEvents::Sensed(SensedEvent {
                agent: selector.agent.clone(),
                content: Content::new(selector.content.as_str()),
                created_at: Timestamp::now(),
            }))
            .await?;

        Ok(ContinuityResponse::Sleeping(dream))
    }

    /// Sleep — end a session, capture continuity.
    pub async fn sleep(
        context: &ProjectContext,
        selector: &SleepAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;

        context
            .emit(ContinuityEvents::Slept(ContinuityEvent {
                agent: selector.agent.clone(),
                created_at: Timestamp::now(),
            }))
            .await?;

        Ok(ContinuityResponse::Sleeping(dream))
    }

    /// Guidebook — gather cognitive context without emitting an event.
    ///
    /// Used to display the agent's full operational context (textures,
    /// sensations, levels, urges) without marking a continuity transition.
    pub fn guidebook(
        context: &ProjectContext,
        selector: &GuidebookAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let dream = Self::gather_context(context, &selector.agent, overrides)?;
        Ok(ContinuityResponse::Guidebook(dream))
    }

    /// Gather the full cognitive context for an agent.
    ///
    /// Uses Store types directly since we hold an owned Connection.
    pub fn gather_context(
        context: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<DreamContext, ContinuityError> {
        let config = context.config.dream.merge(overrides);
        let db = context.db()?;

        let agent = AgentStore::new(&db)
            .get(agent_name)?
            .ok_or_else(|| ContinuityError::AgentNotFound(agent_name.clone()))?;

        let persona_name = agent.persona.clone();
        let persona = PersonaStore::new(&db).get(&persona_name)?;

        let agent_id_str = agent.id.to_string();

        // Vocabulary — system-wide
        let textures = TextureStore::new(&db).list()?;
        let levels = LevelStore::new(&db).list()?;
        let sensations = SensationStore::new(&db).list()?;
        let natures = NatureStore::new(&db).list()?;
        let urges = UrgeStore::new(&db).list()?;

        // Memory filtering — core always included, rest filtered by level + capped
        let all_memories = MemoryStore::new(&db).list(Some(&agent_id_str))?;
        let (core_memories, rest_memories): (Vec<_>, Vec<_>) = all_memories
            .into_iter()
            .partition(|m| m.level.as_str() == "core");

        let filtered_rest = Self::filter_memories(&config, rest_memories);
        let mut memories: Vec<Memory> = core_memories;
        memories.extend(filtered_rest);
        memories.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        // Recent experiences provide orientation
        let recent_experiences =
            ExperienceStore::new(&db).list_recent(&agent_id_str, config.recent_window)?;

        // BFS seed: filtered memories + recent experiences
        let seed_refs: Vec<Ref> = memories
            .iter()
            .map(|m| Ref::memory(m.id))
            .chain(recent_experiences.iter().map(|e| Ref::experience(e.id)))
            .collect();

        // Graph traversal — BFS from foundation through all connection types.
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

            let ref_json = serde_json::to_string(&current).map_err(EventError::from)?;
            let node_connections = ConnectionStore::new(&db).list(Some(&ref_json))?;

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
                let within_depth = config.dream_depth.is_none_or(|max| neighbor_depth <= max);

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
            // No connections means sparse graph — include all cognitions
            CognitionStore::new(&db).list(Some(&agent.id), None)?
        } else {
            Self::assemble_cognitions(&db, &agent, &config, &cognition_ids)?
        };

        // Apply cognition_size cap — keep the most recent.
        if let Some(max) = config.cognition_size
            && cognitions.len() > max
        {
            cognitions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            cognitions = cognitions.split_off(cognitions.len() - max);
        }

        // Assemble experiences: recent + graph-discovered, deduped
        let mut experiences =
            Self::assemble_experiences(&db, &recent_experiences, &experience_ids)?;

        // Apply experience_size cap — keep the most recent.
        if let Some(max) = config.experience_size
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

        // Pressure readings paired with urge CTAs
        let raw_pressures = PressureStore::new(&db).get(agent_name)?;
        let pressures = PressureReading::from_pressures_and_urges(raw_pressures, &urges);

        Ok(DreamContext::builder()
            .agent(agent)
            .maybe_persona(persona)
            .memories(memories)
            .cognitions(cognitions)
            .experiences(experiences)
            .connections(connections)
            .textures(textures)
            .levels(levels)
            .sensations(sensations)
            .natures(natures)
            .urges(urges)
            .pressures(pressures)
            .build())
    }

    fn filter_memories(config: &DreamConfig, mut memories: Vec<Memory>) -> Vec<Memory> {
        // Filter by level threshold (log-level semantics)
        if let Some(threshold) = &config.recollection_level {
            let min_priority = level_priority(threshold);
            memories.retain(|m| level_priority(&m.level) >= min_priority);
        }

        // Sort by created_at descending for recency-based capping
        memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Cap at recollection_size
        if let Some(max) = config.recollection_size {
            memories.truncate(max);
        }

        memories
    }

    fn assemble_cognitions(
        db: &rusqlite::Connection,
        agent: &Agent,
        config: &DreamConfig,
        graph_ids: &HashSet<CognitionId>,
    ) -> Result<Vec<Cognition>, ContinuityError> {
        let store = CognitionStore::new(db);

        // Recent cognitions for orientation
        let recent = store.list_recent(&agent.id, config.recent_window)?;

        let mut cognitions: Vec<Cognition> = Vec::new();
        let mut seen: HashSet<CognitionId> = HashSet::new();

        // Recent first
        for cog in recent {
            if seen.insert(cog.id) {
                cognitions.push(cog);
            }
        }

        // Graph-discovered
        for id in graph_ids {
            if seen.insert(*id)
                && let Some(cog) = store.get(id)?
            {
                cognitions.push(cog);
            }
        }

        cognitions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(cognitions)
    }

    fn assemble_experiences(
        db: &rusqlite::Connection,
        recent: &[Experience],
        graph_ids: &HashSet<ExperienceId>,
    ) -> Result<Vec<Experience>, ContinuityError> {
        let store = ExperienceStore::new(db);
        let mut experiences: Vec<Experience> = Vec::new();
        let mut seen: HashSet<ExperienceId> = HashSet::new();

        // Recent first
        for exp in recent {
            if seen.insert(exp.id) {
                experiences.push(exp.clone());
            }
        }

        // Graph-discovered
        for id in graph_ids {
            if seen.insert(*id)
                && let Some(exp) = store.get(id)?
            {
                experiences.push(exp);
            }
        }

        experiences.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(experiences)
    }
}
