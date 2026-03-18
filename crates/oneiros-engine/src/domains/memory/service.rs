use chrono::Utc;
use uuid::Uuid;

use crate::contexts::ProjectContext;

use super::errors::MemoryError;
use super::model::Memory;
use super::repo::MemoryRepo;
use super::responses::MemoryResponse;

pub struct MemoryService;

impl MemoryService {
    pub fn add(
        ctx: &ProjectContext,
        agent: String,
        level: String,
        content: String,
    ) -> Result<MemoryResponse, MemoryError> {
        let memory = Memory {
            id: Uuid::now_v7().to_string(),
            agent_id: agent,
            level,
            content,
            created_at: Utc::now().to_rfc3339(),
        };

        ctx.emit("memory-added", &memory);
        Ok(MemoryResponse::Added(memory))
    }

    pub fn get(ctx: &ProjectContext, id: &str) -> Result<MemoryResponse, MemoryError> {
        let memory = ctx
            .with_db(|conn| MemoryRepo::new(conn).get(id))
            .map_err(MemoryError::Database)?
            .ok_or_else(|| MemoryError::NotFound(id.to_string()))?;
        Ok(MemoryResponse::Found(memory))
    }

    pub fn list(ctx: &ProjectContext, agent: Option<&str>) -> Result<MemoryResponse, MemoryError> {
        let memories = ctx
            .with_db(|conn| MemoryRepo::new(conn).list(agent))
            .map_err(MemoryError::Database)?;
        Ok(MemoryResponse::Listed(memories))
    }
}
