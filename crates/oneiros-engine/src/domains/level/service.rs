use crate::*;

pub struct LevelService;

impl LevelService {
    pub async fn set(context: &ProjectContext, level: Level) -> Result<LevelResponse, LevelError> {
        let name = level.name.clone();
        context.emit(LevelEvents::LevelSet(level)).await?;
        Ok(LevelResponse::LevelSet(name))
    }

    pub async fn get(
        context: &ProjectContext,
        name: &LevelName,
    ) -> Result<LevelResponse, LevelError> {
        let level = LevelRepo::new(context)
            .get(name)
            .await?
            .ok_or_else(|| LevelError::NotFound(name.clone()))?;
        Ok(LevelResponse::LevelDetails(level))
    }

    pub async fn list(context: &ProjectContext) -> Result<LevelResponse, LevelError> {
        let levels = LevelRepo::new(context).list().await?;
        if levels.is_empty() {
            Ok(LevelResponse::NoLevels)
        } else {
            Ok(LevelResponse::Levels(levels))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        name: &LevelName,
    ) -> Result<LevelResponse, LevelError> {
        context
            .emit(LevelEvents::LevelRemoved(LevelRemoved {
                name: name.clone(),
            }))
            .await?;
        Ok(LevelResponse::LevelRemoved(name.clone()))
    }
}
