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
        // Verify agent exists
        ctx.with_db(|conn| AgentRepo::new(conn).get(agent_name))?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.clone()))?;

        ctx.emit(LifecycleEvents::Sensed(SensedEvent {
            agent: agent_name.clone(),
            content: Content::new(content.as_str()),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Sleeping {
            agent: agent_name.to_string(),
        })
    }

    /// Sleep — end a session, capture continuity.
    pub fn sleep(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<LifecycleResponse, LifecycleError> {
        // Verify agent exists
        ctx.with_db(|conn| AgentRepo::new(conn).get(agent_name))?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.clone()))?;

        ctx.emit(LifecycleEvents::Slept(LifecycleEvent {
            agent: agent_name.clone(),
            created_at: Timestamp::now(),
        }));

        Ok(LifecycleResponse::Sleeping {
            agent: agent_name.to_string(),
        })
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
    pub fn gather_context(
        ctx: &ProjectContext,
        agent_name: &AgentName,
    ) -> Result<CognitiveContext, LifecycleError> {
        let agent = ctx
            .with_db(|conn| AgentRepo::new(conn).get(agent_name))?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.clone()))?;

        let agent_id_str = agent.id.to_string();

        let cognitions = ctx
            .with_db(|conn| CognitionRepo::new(conn).list(Some(&agent_id_str), None))
            .map_err(LifecycleError::Database)?;

        let memories = ctx
            .with_db(|conn| MemoryRepo::new(conn).list(Some(&agent_id_str)))
            .map_err(LifecycleError::Database)?;

        let experiences = ctx
            .with_db(|conn| ExperienceRepo::new(conn).list(Some(&agent_id_str)))
            .map_err(LifecycleError::Database)?;

        Ok(CognitiveContext {
            agent,
            cognitions,
            memories,
            experiences,
        })
    }
}
