use crate::*;

pub struct LevelService;

impl LevelService {
    pub fn set(context: &ProjectContext, level: Level) -> Result<LevelResponse, LevelError> {
        let name = level.name.clone();
        context.emit(LevelEvents::LevelSet(level));
        Ok(LevelResponse::LevelSet(name))
    }

    pub fn get(context: &ProjectContext, name: &LevelName) -> Result<LevelResponse, LevelError> {
        let level = context
            .with_db(|conn| LevelRepo::new(conn).get(name))
            .map_err(LevelError::Database)?
            .ok_or_else(|| LevelError::NotFound(name.clone()))?;
        Ok(LevelResponse::LevelDetails(level))
    }

    pub fn list(context: &ProjectContext) -> Result<LevelResponse, LevelError> {
        let levels = context
            .with_db(|conn| LevelRepo::new(conn).list())
            .map_err(LevelError::Database)?;
        if levels.is_empty() {
            Ok(LevelResponse::NoLevels)
        } else {
            Ok(LevelResponse::Levels(levels))
        }
    }

    pub fn remove(context: &ProjectContext, name: &LevelName) -> Result<LevelResponse, LevelError> {
        context.emit(LevelEvents::LevelRemoved(LevelRemoved {
            name: name.clone(),
        }));
        Ok(LevelResponse::LevelRemoved(name.clone()))
    }
}
