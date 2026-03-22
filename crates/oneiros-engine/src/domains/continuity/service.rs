//! Continuity service — composes other domain services into workflows.
//!
//! These operations don't have their own repos. They gather data from
//! multiple domains, perform the operation, and emit continuity events.

use crate::*;

pub struct ContinuityService;

impl ContinuityService {
    /// Emerge — create an agent and immediately activate its continuity.
    pub fn emerge(
        ctx: &ProjectContext,
        name: AgentName,
        persona: PersonaName,
        description: Description,
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
        Self::wake(ctx, &agent_name)?;
        let context = Self::gather_context(ctx, &agent_name)?;
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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;
        Ok(ContinuityResponse::Status(context))
    }

    /// Wake — restore an agent's full cognitive context (initial session start).
    pub fn wake(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;

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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;

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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;

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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;

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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;

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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;

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
    ) -> Result<ContinuityResponse, ContinuityError> {
        let context = Self::gather_context(ctx, agent_name)?;
        Ok(ContinuityResponse::Guidebook(context))
    }

    /// Gather the full cognitive context for an agent.
    ///
    /// Assembles everything needed for identity reconstruction: the agent itself,
    /// its persona, all cognitive records, the full vocabulary, graph connections,
    /// and pressure readings.
    pub fn gather_context(
        context: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<CognitiveContext, ContinuityError> {
        let agent = context
            .with_db(|conn| AgentRepo::new(conn).get(agent_name))?
            .ok_or_else(|| ContinuityError::AgentNotFound(agent_name.clone()))?;

        let agent_id_str = agent.id.to_string();
        let persona_name = agent.persona.clone();

        Ok(CognitiveContext {
            agent,
            persona: context.with_db(|conn| PersonaRepo::new(conn).get(&persona_name))?,
            cognitions: context
                .with_db(|conn| CognitionRepo::new(conn).list(Some(&agent_id_str), None))?,
            memories: context.with_db(|conn| MemoryRepo::new(conn).list(Some(&agent_id_str)))?,
            experiences: context
                .with_db(|conn| ExperienceRepo::new(conn).list(Some(&agent_id_str)))?,
            connections: context.with_db(|conn| ConnectionRepo::new(conn).list(None))?,
            textures: context.with_db(|conn| TextureRepo::new(conn).list())?,
            levels: context.with_db(|conn| LevelRepo::new(conn).list())?,
            sensations: context.with_db(|conn| SensationRepo::new(conn).list())?,
            natures: context.with_db(|conn| NatureRepo::new(conn).list())?,
            urges: context.with_db(|conn| UrgeRepo::new(conn).list())?,
            pressures: context.with_db(|conn| PressureRepo::new(conn).get(agent_name))?,
        })
    }
}
