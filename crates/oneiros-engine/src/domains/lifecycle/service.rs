//! Lifecycle service — composes other domain services into workflows.
//!
//! These operations don't have their own repos. They gather data from
//! multiple domains, perform the operation, and emit lifecycle events.

use crate::*;

pub struct LifecycleService;

impl LifecycleService {
    /// Wake — restore an agent's full cognitive context (initial session start).
    pub fn wake(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(LifecycleEvents::Dreamed(LifecycleEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Waking(context))
    }

    /// Dream — restore an agent's full cognitive context.
    pub fn dream(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(LifecycleEvents::Dreamed(LifecycleEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Dreaming(context))
    }

    /// Introspect — look inward, consolidate cognitive state.
    pub fn introspect(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(LifecycleEvents::Introspected(LifecycleEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Introspecting(context))
    }

    /// Reflect — pause on something significant.
    pub fn reflect(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(LifecycleEvents::Reflected(LifecycleEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Reflecting(context))
    }

    /// Sense — receive and interpret something from outside.
    pub fn sense(
        ctx: &ProjectContext,
        agent_name: &AgentName,
        content: &Content,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(LifecycleEvents::Sensed(SensedEvent {
            agent: agent_name.clone(),
            content: Content::new(content.as_str()),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Sleeping(context))
    }

    /// Sleep — end a session, capture continuity.
    pub fn sleep(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(LifecycleEvents::Slept(LifecycleEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Sleeping(context))
    }

    /// Guidebook — gather cognitive context without emitting an event.
    ///
    /// Used to display the agent's full operational context (textures,
    /// sensations, levels, urges) without marking a lifecycle transition.
    pub fn guidebook(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;
        Ok(LifecycleResponse::Guidebook(context))
    }

    /// Gather the full cognitive context for an agent.
    ///
    /// Assembles everything needed for identity reconstruction: the agent itself,
    /// its persona, all cognitive records, the full vocabulary, graph connections,
    /// and pressure readings.
    pub fn gather_context(
        context: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<CognitiveContext, LifecycleError> {
        let agent = context
            .with_db(|conn| AgentRepo::new(conn).get(agent_name))?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.clone()))?;

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
