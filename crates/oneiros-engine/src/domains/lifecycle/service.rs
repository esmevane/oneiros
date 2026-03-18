//! Lifecycle service — composes other domain services into workflows.
//!
//! These operations don't have their own repos. They gather data from
//! multiple domains, perform the operation, and emit lifecycle events.

use chrono::Utc;

use crate::contexts::ProjectContext;
use crate::domains::agent::repo::AgentRepo;
use crate::domains::cognition::repo::CognitionRepo;
use crate::domains::experience::repo::ExperienceRepo;
use crate::domains::memory::repo::MemoryRepo;

use super::errors::LifecycleError;
use super::model::{CognitiveContext, LifecycleMarker};
use super::responses::LifecycleResponse;

pub struct LifecycleService;

impl LifecycleService {
    /// Dream — restore an agent's full cognitive context.
    pub fn dream(
        ctx: &ProjectContext,
        agent_name: &str,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(
            "dreamed",
            &LifecycleMarker {
                agent: agent_name.to_string(),
                operation: "dream".to_string(),
                created_at: Utc::now().to_rfc3339(),
            },
        );

        Ok(LifecycleResponse::Dreamed(context))
    }

    /// Introspect — look inward, consolidate cognitive state.
    pub fn introspect(
        ctx: &ProjectContext,
        agent_name: &str,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(
            "introspected",
            &LifecycleMarker {
                agent: agent_name.to_string(),
                operation: "introspect".to_string(),
                created_at: Utc::now().to_rfc3339(),
            },
        );

        Ok(LifecycleResponse::Introspected(context))
    }

    /// Reflect — pause on something significant.
    pub fn reflect(
        ctx: &ProjectContext,
        agent_name: &str,
    ) -> Result<LifecycleResponse, LifecycleError> {
        let context = Self::gather_context(ctx, agent_name)?;

        ctx.emit(
            "reflected",
            &LifecycleMarker {
                agent: agent_name.to_string(),
                operation: "reflect".to_string(),
                created_at: Utc::now().to_rfc3339(),
            },
        );

        Ok(LifecycleResponse::Reflected(context))
    }

    /// Sense — receive and interpret something from outside.
    pub fn sense(
        ctx: &ProjectContext,
        agent_name: &str,
        content: &str,
    ) -> Result<LifecycleResponse, LifecycleError> {
        // Verify agent exists
        ctx.with_db(|conn| AgentRepo::new(conn).get(agent_name))
            .map_err(LifecycleError::Database)?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.to_string()))?;

        ctx.emit(
            "sensed",
            &serde_json::json!({
                "agent": agent_name,
                "content": content,
                "created_at": Utc::now().to_rfc3339(),
            }),
        );

        Ok(LifecycleResponse::Sensed {
            agent: agent_name.to_string(),
        })
    }

    /// Sleep — end a session, capture continuity.
    pub fn sleep(
        ctx: &ProjectContext,
        agent_name: &str,
    ) -> Result<LifecycleResponse, LifecycleError> {
        // Verify agent exists
        ctx.with_db(|conn| AgentRepo::new(conn).get(agent_name))
            .map_err(LifecycleError::Database)?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.to_string()))?;

        ctx.emit(
            "slept",
            &LifecycleMarker {
                agent: agent_name.to_string(),
                operation: "sleep".to_string(),
                created_at: Utc::now().to_rfc3339(),
            },
        );

        Ok(LifecycleResponse::Slept {
            agent: agent_name.to_string(),
        })
    }

    /// Gather the full cognitive context for an agent.
    fn gather_context(
        ctx: &ProjectContext,
        agent_name: &str,
    ) -> Result<CognitiveContext, LifecycleError> {
        let agent = ctx
            .with_db(|conn| AgentRepo::new(conn).get(agent_name))
            .map_err(LifecycleError::Database)?
            .ok_or_else(|| LifecycleError::AgentNotFound(agent_name.to_string()))?;

        let cognitions = ctx
            .with_db(|conn| CognitionRepo::new(conn).list(Some(agent_name), None))
            .map_err(LifecycleError::Database)?;

        let memories = ctx
            .with_db(|conn| MemoryRepo::new(conn).list(Some(agent_name)))
            .map_err(LifecycleError::Database)?;

        let experiences = ctx
            .with_db(|conn| ExperienceRepo::new(conn).list(Some(agent_name)))
            .map_err(LifecycleError::Database)?;

        Ok(CognitiveContext {
            agent,
            cognitions,
            memories,
            experiences,
        })
    }
}
