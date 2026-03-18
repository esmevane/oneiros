use crate::*;

pub struct LevelService;

impl LevelService {
    pub fn set(ctx: &ProjectContext, level: Level) -> Result<LevelResponse, LevelError> {
        ctx.emit(LevelEvents::LevelSet(level.clone()));
        Ok(LevelResponse::Set(level))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<LevelResponse, LevelError> {
        let level = ctx
            .with_db(|conn| LevelRepo::new(conn).get(name))
            .map_err(LevelError::Database)?
            .ok_or_else(|| LevelError::NotFound(name.to_string()))?;
        Ok(LevelResponse::Found(level))
    }

    pub fn list(ctx: &ProjectContext) -> Result<LevelResponse, LevelError> {
        let levels = ctx
            .with_db(|conn| LevelRepo::new(conn).list())
            .map_err(LevelError::Database)?;
        Ok(LevelResponse::Listed(levels))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<LevelResponse, LevelError> {
        ctx.emit(LevelEvents::LevelRemoved(LevelRemoved {
            name: LevelName::new(name),
        }));
        Ok(LevelResponse::Removed)
    }
}
