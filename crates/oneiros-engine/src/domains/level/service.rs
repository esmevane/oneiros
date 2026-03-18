use crate::*;

pub struct LevelService;

impl LevelService {
    pub fn set(ctx: &ProjectContext, level: Level) -> Result<LevelResponse, LevelError> {
        let name = level.name.clone();
        ctx.emit(LevelEvents::LevelSet(level));
        Ok(LevelResponse::LevelSet(name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<LevelResponse, LevelError> {
        let level = ctx
            .with_db(|conn| LevelRepo::new(conn).get(name))
            .map_err(LevelError::Database)?
            .ok_or_else(|| LevelError::NotFound(name.to_string()))?;
        Ok(LevelResponse::LevelDetails(level))
    }

    pub fn list(ctx: &ProjectContext) -> Result<LevelResponse, LevelError> {
        let levels = ctx
            .with_db(|conn| LevelRepo::new(conn).list())
            .map_err(LevelError::Database)?;
        if levels.is_empty() {
            Ok(LevelResponse::NoLevels)
        } else {
            Ok(LevelResponse::Levels(levels))
        }
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<LevelResponse, LevelError> {
        let level_name = LevelName::new(name);
        ctx.emit(LevelEvents::LevelRemoved(LevelRemoved {
            name: level_name.clone(),
        }));
        Ok(LevelResponse::LevelRemoved(level_name))
    }
}
