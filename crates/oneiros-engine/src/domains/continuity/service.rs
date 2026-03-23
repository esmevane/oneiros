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
    pub fn emerge(
        ctx: &ProjectContext,
        name: AgentName,
        persona: PersonaName,
        description: Description,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let created = AgentService::create(ctx, name, persona, description, Prompt::new(""))?;

        let agent_name = match created {
            AgentResponse::AgentCreated(n) => n,
            other => {
                return Err(ContinuityError::AgentNotFound(AgentName::new(format!(
                    "unexpected: {other:?}"
                ))));
            }
        };

        // Wake activates continuity; then gather the full context for the response.
        Self::wake(ctx, &agent_name, overrides)?;
        let context = Self::gather_context(ctx, &agent_name, overrides)?;
        Ok(ContinuityResponse::Emerged(context))
    }

    /// Recede — retire an agent, ending its continuity.
    pub fn recede(
        ctx: &ProjectContext,
        name: &AgentName,
    ) -> Result<ContinuityResponse, ContinuityError> {
        AgentService::remove(ctx, name)?;
        Ok(ContinuityResponse::Receded(name.clone()))
    }

    /// Status — read the current state of an agent's continuity.
    pub fn status(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;
        Ok(ContinuityResponse::Status(context))
    }

    /// Wake — restore an agent's full cognitive context (initial session start).
    pub fn wake(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;

        ctx.emit(ContinuityEvents::Dreamed(ContinuityEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(ContinuityResponse::Waking(context))
    }

    /// Dream — restore an agent's full cognitive context.
    pub fn dream(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;

        ctx.emit(ContinuityEvents::Dreamed(ContinuityEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(ContinuityResponse::Dreaming(context))
    }

    /// Introspect — look inward, consolidate cognitive state.
    pub fn introspect(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;

        ctx.emit(ContinuityEvents::Introspected(ContinuityEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(ContinuityResponse::Introspecting(context))
    }

    /// Reflect — pause on something significant.
    pub fn reflect(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;

        ctx.emit(ContinuityEvents::Reflected(ContinuityEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(ContinuityResponse::Reflecting(context))
    }

    /// Sense — receive and interpret something from outside.
    pub fn sense(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        content: &Content,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;

        ctx.emit(ContinuityEvents::Sensed(SensedEvent {
            agent: agent_name.clone(),
            content: Content::new(content.as_str()),
            created_at: Timestamp::now(),
        }));

        Ok(ContinuityResponse::Sleeping(context))
    }

    /// Sleep — end a session, capture continuity.
    pub fn sleep(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;

        ctx.emit(ContinuityEvents::Slept(ContinuityEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(ContinuityResponse::Sleeping(context))
    }

    /// Guidebook — gather cognitive context without emitting an event.
    ///
    /// Used to display the agent's full operational context (textures,
    /// sensations, levels, urges) without marking a continuity transition.
    pub fn guidebook(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name, overrides)?;
        Ok(ContinuityResponse::Guidebook(context))
    }

    /// Gather the full cognitive context for an agent.
    ///
    /// Assembles everything needed for identity reconstruction: the agent itself,
    /// its persona, all cognitive records, the full vocabulary, graph connections,
    /// and pressure readings.
    ///
    /// Uses BFS graph traversal from seed entities (filtered memories + recent
    /// experiences) to discover connected cognitions and experiences. Falls back
    /// to listing all cognitions when the graph is sparse (no connections).
    pub fn gather_context(
        context: &ProjectContext,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<DreamContext, ContinuityError> {
        let config = context.dream_config().merge(overrides);

        let agent = context
            .with_db(|conn| AgentRepo::new(conn).get(agent_name))?
            .ok_or_else(|| ContinuityError::AgentNotFound(agent_name.clone()))?;

        let persona_name = agent.persona.clone();
        let persona = context.with_db(|conn| PersonaRepo::new(conn).get(&persona_name))?;

        let agent_id_str = agent.id.to_string();

        // Vocabulary — system-wide
        let textures = context.with_db(|conn| TextureRepo::new(conn).list())?;
        let levels = context.with_db(|conn| LevelRepo::new(conn).list())?;
        let sensations = context.with_db(|conn| SensationRepo::new(conn).list())?;
        let natures = context.with_db(|conn| NatureRepo::new(conn).list())?;
        let urges = context.with_db(|conn| UrgeRepo::new(conn).list())?;

        // Memory filtering — core always included, rest filtered by level + capped
        let all_memories =
            context.with_db(|conn| MemoryRepo::new(conn).list(Some(&agent_id_str)))?;
        let (core_memories, rest_memories): (Vec<_>, Vec<_>) = all_memories
            .into_iter()
            .partition(|m| m.level.as_str() == "core");

        let filtered_rest = Self::filter_memories(&config, rest_memories);
        let mut memories: Vec<Memory> = core_memories;
        memories.extend(filtered_rest);
        memories.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        // Recent experiences provide orientation
        let recent_experiences = context.with_db(|conn| {
            ExperienceRepo::new(conn).list_recent(&agent_id_str, config.recent_window)
        })?;

        // BFS seed: filtered memories + recent experiences
        let seed_refs: Vec<Ref> = memories
            .iter()
            .map(|m| Ref::memory(m.id))
            .chain(recent_experiences.iter().map(|e| Ref::experience(e.id)))
            .collect();

        // Graph traversal — BFS from foundation through all connection types.
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

            let ref_json = serde_json::to_string(&current).map_err(EventError::from)?;
            let node_connections =
                context.with_db(|conn| ConnectionRepo::new(conn).list(Some(&ref_json)))?;

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
            context.with_db(|conn| CognitionRepo::new(conn).list(Some(&agent_id_str), None))?
        } else {
            Self::assemble_cognitions(context, &agent, &config, &cognition_ids)?
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
            Self::assemble_experiences(context, &recent_experiences, &experience_ids)?;

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
        let raw_pressures = context.with_db(|conn| PressureRepo::new(conn).get(agent_name))?;
        let pressures = PressureReading::from_pressures_and_urges(raw_pressures, &urges);

        Ok(DreamContext {
            agent,
            persona,
            memories,
            cognitions,
            experiences,
            connections,
            textures,
            levels,
            sensations,
            natures,
            urges,
            pressures,
        })
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
        context: &ProjectContext,
        agent: &Agent,
        config: &DreamConfig,
        graph_ids: &HashSet<CognitionId>,
    ) -> Result<Vec<Cognition>, ContinuityError> {
        let agent_id_str = agent.id.to_string();

        // Recent cognitions for orientation
        let recent = context.with_db(|conn| {
            CognitionRepo::new(conn).list_recent(&agent_id_str, config.recent_window)
        })?;

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
            if seen.insert(*id) {
                if let Some(cog) = context.with_db(|conn| CognitionRepo::new(conn).get(id))? {
                    cognitions.push(cog);
                }
            }
        }

        cognitions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(cognitions)
    }

    fn assemble_experiences(
        context: &ProjectContext,
        recent: &[Experience],
        graph_ids: &HashSet<ExperienceId>,
    ) -> Result<Vec<Experience>, ContinuityError> {
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
            if seen.insert(*id) {
                if let Some(exp) = context.with_db(|conn| ExperienceRepo::new(conn).get(id))? {
                    experiences.push(exp);
                }
            }
        }

        experiences.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(experiences)
    }
}
