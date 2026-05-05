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
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &EmergeAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let EmergeAgent::V1(emerging) = request;
        let created = AgentService::create(
            scope,
            mailbox,
            &CreateAgent::builder_v1()
                .name(emerging.name.clone())
                .persona(emerging.persona.clone())
                .description(emerging.description.clone())
                .build()
                .into(),
        )
        .await?;

        let created_agent = match created {
            AgentResponse::AgentCreated(AgentCreatedResponse::V1(created)) => created.agent,
            other => {
                return Err(ContinuityError::UnexpectedResponse(format!("{other:?}")));
            }
        };

        // Wake activates continuity; then gather the full context for
        // the response. We pass the agent record we already hold
        // rather than re-looking-up by name — the lookup race doesn't
        // happen because the lookup doesn't happen.
        Self::wake(
            scope,
            mailbox,
            &WakeAgent::builder_v1()
                .agent(created_agent.name.clone())
                .build()
                .into(),
            overrides,
        )
        .await?;
        let dream = Self::gather_context_for(scope, &created_agent, overrides).await?;
        Ok(ContinuityResponse::Emerged(
            EmergedResponse::builder_v1().context(dream).build().into(),
        ))
    }

    /// Recede — retire an agent, ending its continuity.
    pub async fn recede(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RecedeAgent,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let RecedeAgent::V1(receding) = request;
        AgentService::remove(
            scope,
            mailbox,
            &RemoveAgent::builder_v1()
                .name(receding.agent.clone())
                .build()
                .into(),
        )
        .await?;
        Ok(ContinuityResponse::Receded(
            RecededResponse::builder_v1()
                .agent(receding.agent.clone())
                .build()
                .into(),
        ))
    }

    /// Status — cross-agent activity overview.
    pub async fn status(
        scope: &Scope<AtBookmark>,
        _request: &StatusAgent,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let db = BookmarkDb::open(scope).await?;
        let agents = AgentStore::new(&db).list()?;

        let mut rows = Vec::with_capacity(agents.len());

        for agent in &agents {
            let agent_id = agent.id.to_string();

            let cognition_count: usize = db.query_row(
                "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1",
                rusqlite::params![agent_id],
                |row| row.get(0),
            )?;

            let cognition_latest: Option<String> = db
                .query_row(
                    "SELECT created_at FROM cognitions WHERE agent_id = ?1 ORDER BY created_at DESC LIMIT 1",
                    rusqlite::params![agent_id],
                    |row| row.get(0),
                )
                .ok();

            let memory_count: usize = db.query_row(
                "SELECT COUNT(*) FROM memories WHERE agent_id = ?1",
                rusqlite::params![agent_id],
                |row| row.get(0),
            )?;

            let memory_latest: Option<String> = db
                .query_row(
                    "SELECT created_at FROM memories WHERE agent_id = ?1 ORDER BY created_at DESC LIMIT 1",
                    rusqlite::params![agent_id],
                    |row| row.get(0),
                )
                .ok();

            let experience_count: usize = db.query_row(
                "SELECT COUNT(*) FROM experiences WHERE agent_id = ?1",
                rusqlite::params![agent_id],
                |row| row.get(0),
            )?;

            let experience_latest: Option<String> = db
                .query_row(
                    "SELECT created_at FROM experiences WHERE agent_id = ?1 ORDER BY created_at DESC LIMIT 1",
                    rusqlite::params![agent_id],
                    |row| row.get(0),
                )
                .ok();

            rows.push(AgentActivity {
                name: agent.name.clone(),
                cognition_count,
                cognition_latest: cognition_latest.and_then(|s| Timestamp::parse_str(&s).ok()),
                memory_count,
                memory_latest: memory_latest.and_then(|s| Timestamp::parse_str(&s).ok()),
                experience_count,
                experience_latest: experience_latest.and_then(|s| Timestamp::parse_str(&s).ok()),
            });
        }

        Ok(ContinuityResponse::Status(
            StatusResponse::builder_v1()
                .table(AgentActivityTable { agents: rows })
                .build()
                .into(),
        ))
    }

    /// Wake — restore an agent's full cognitive context (initial session start).
    pub async fn wake(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &WakeAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let WakeAgent::V1(wake) = request;
        let dream = Self::gather_context(scope, &wake.agent, overrides).await?;

        let new_event = NewEvent::builder()
            .data(Events::Continuity(ContinuityEvents::Dreamed(
                Dreamed::builder_v1()
                    .agent(wake.agent.clone())
                    .created_at(Timestamp::now())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        Ok(ContinuityResponse::Waking(
            WakingResponse::builder_v1().context(dream).build().into(),
        ))
    }

    /// Dream — restore an agent's full cognitive context.
    pub async fn dream(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &DreamAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let DreamAgent::V1(dreaming) = request;
        let dream = Self::gather_context(scope, &dreaming.agent, overrides).await?;

        let new_event = NewEvent::builder()
            .data(Events::Continuity(ContinuityEvents::Dreamed(
                Dreamed::builder_v1()
                    .agent(dreaming.agent.clone())
                    .created_at(Timestamp::now())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        Ok(ContinuityResponse::Dreaming(
            DreamingResponse::builder_v1().context(dream).build().into(),
        ))
    }

    /// Introspect — look inward, consolidate cognitive state.
    pub async fn introspect(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &IntrospectAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let IntrospectAgent::V1(introspecting) = request;
        let dream = Self::gather_context(scope, &introspecting.agent, overrides).await?;

        let new_event = NewEvent::builder()
            .data(Events::Continuity(ContinuityEvents::Introspected(
                Introspected::builder_v1()
                    .agent(introspecting.agent.clone())
                    .created_at(Timestamp::now())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        Ok(ContinuityResponse::Introspecting(
            IntrospectingResponse::builder_v1()
                .context(dream)
                .build()
                .into(),
        ))
    }

    /// Reflect — pause on something significant.
    pub async fn reflect(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &ReflectAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let ReflectAgent::V1(reflecting) = request;
        let dream = Self::gather_context(scope, &reflecting.agent, overrides).await?;

        let new_event = NewEvent::builder()
            .data(Events::Continuity(ContinuityEvents::Reflected(
                Reflected::builder_v1()
                    .agent(reflecting.agent.clone())
                    .created_at(Timestamp::now())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        Ok(ContinuityResponse::Reflecting(
            ReflectingResponse::builder_v1()
                .context(dream)
                .build()
                .into(),
        ))
    }

    /// Sense — receive and interpret something from outside.
    pub async fn sense(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SenseContent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let SenseContent::V1(sensing) = request;
        let dream = Self::gather_context(scope, &sensing.agent, overrides).await?;

        let new_event = NewEvent::builder()
            .data(Events::Continuity(ContinuityEvents::Sensed(
                Sensed::builder_v1()
                    .agent(sensing.agent.clone())
                    .content(Content::new(sensing.content.as_str()))
                    .created_at(Timestamp::now())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        Ok(ContinuityResponse::Sleeping(
            SleepingResponse::builder_v1().context(dream).build().into(),
        ))
    }

    /// Sleep — end a session, capture continuity.
    pub async fn sleep(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SleepAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let SleepAgent::V1(sleeping) = request;
        let dream = Self::gather_context(scope, &sleeping.agent, overrides).await?;

        let new_event = NewEvent::builder()
            .data(Events::Continuity(ContinuityEvents::Slept(
                Slept::builder_v1()
                    .agent(sleeping.agent.clone())
                    .created_at(Timestamp::now())
                    .build()
                    .into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        Ok(ContinuityResponse::Sleeping(
            SleepingResponse::builder_v1().context(dream).build().into(),
        ))
    }

    /// Guidebook — gather cognitive context without emitting an event.
    ///
    /// Used to display the agent's full operational context (textures,
    /// sensations, levels, urges) without marking a continuity transition.
    pub async fn guidebook(
        scope: &Scope<AtBookmark>,
        request: &GuidebookAgent,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let GuidebookAgent::V1(lookup) = request;
        let dream = Self::gather_context(scope, &lookup.agent, overrides).await?;
        Ok(ContinuityResponse::Guidebook(
            GuidebookResponse::builder_v1()
                .context(dream)
                .build()
                .into(),
        ))
    }

    /// Gather the full cognitive context for an agent by name.
    ///
    /// Resolves the agent record patiently via [`AgentRepo::fetch`], then
    /// delegates to [`gather_context_for`].
    ///
    /// [`gather_context_for`]: ContinuityService::gather_context_for
    pub async fn gather_context(
        scope: &Scope<AtBookmark>,
        agent_name: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<DreamContext, ContinuityError> {
        let agent = AgentRepo::new(scope)
            .fetch(agent_name)
            .await?
            .ok_or_else(|| ContinuityError::AgentNotFound(agent_name.clone()))?;
        Self::gather_context_for(scope, &agent, overrides).await
    }

    /// Gather the full cognitive context for a known agent.
    ///
    /// Takes the agent record directly — used by composers that already
    /// hold the agent (e.g. [`emerge`] passes the agent it just created)
    /// to avoid the re-lookup race. Uses Store types directly since we
    /// hold an owned Connection.
    ///
    /// [`emerge`]: ContinuityService::emerge
    pub async fn gather_context_for(
        scope: &Scope<AtBookmark>,
        agent: &Agent,
        overrides: &DreamOverrides,
    ) -> Result<DreamContext, ContinuityError> {
        let config = scope.config().dream.merge(overrides);
        let db = BookmarkDb::open(scope).await?;

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
        memories.sort_by_key(|a| a.created_at);

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
                let within_depth = neighbor_depth <= config.dream_depth;

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
            Self::assemble_cognitions(&db, agent, &config, &cognition_ids)?
        };

        // Apply cognition_size cap — keep the most recent.
        if cognitions.len() > config.cognition_size {
            cognitions.sort_by_key(|a| a.created_at);
            cognitions = cognitions.split_off(cognitions.len() - config.cognition_size);
        }

        // Assemble experiences: recent + graph-discovered, deduped
        let mut experiences =
            Self::assemble_experiences(&db, &recent_experiences, &experience_ids)?;

        // Apply experience_size cap — keep the most recent.
        if experiences.len() > config.experience_size {
            experiences.sort_by_key(|a| a.created_at);
            experiences = experiences.split_off(experiences.len() - config.experience_size);
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
        let raw_pressures = PressureStore::new(&db).get(&agent.name)?;
        let pressures = PressureReading::from_pressures_and_urges(raw_pressures, &urges);

        Ok(DreamContext::builder()
            .agent(agent.clone())
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
        let min_priority = level_priority(&config.recollection_level);
        memories.retain(|m| level_priority(&m.level) >= min_priority);

        // Sort by created_at descending for recency-based capping
        memories.sort_by_key(|b| std::cmp::Reverse(b.created_at));

        // Cap at recollection_size
        memories.truncate(config.recollection_size);

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

        cognitions.sort_by_key(|a| a.created_at);
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

        experiences.sort_by_key(|a| a.created_at);
        Ok(experiences)
    }
}
