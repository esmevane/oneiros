use crate::*;

pub struct LevelService;

impl LevelService {
    pub async fn set(
        context: &ProjectContext,
        SetLevel {
            name,
            description,
            prompt,
        }: &SetLevel,
    ) -> Result<LevelResponse, LevelError> {
        let level = Level::Current(
            Level::build_v1()
                .name(name.clone())
                .description(description.clone())
                .prompt(prompt.clone())
                .build(),
        );
        context.emit(LevelEvents::LevelSet(level)).await?;
        Ok(LevelResponse::LevelSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetLevel,
    ) -> Result<LevelResponse, LevelError> {
        let name = selector.key.resolve()?;
        let level = LevelRepo::new(context)
            .get(&name)
            .await?
            .ok_or(LevelError::NotFound(name))?;
        let ref_token = RefToken::new(Ref::level(level.name().clone()));
        Ok(LevelResponse::LevelDetails(
            Response::new(level).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListLevels { filters }: &ListLevels,
    ) -> Result<LevelResponse, LevelError> {
        let listed = LevelRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(LevelResponse::NoLevels)
        } else {
            Ok(LevelResponse::Levels(listed.map(|e| {
                let ref_token = RefToken::new(Ref::level(e.name().clone()));
                Response::new(e).with_ref_token(ref_token)
            })))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveLevel,
    ) -> Result<LevelResponse, LevelError> {
        context
            .emit(LevelEvents::LevelRemoved(LevelRemoved::Current(
                LevelRemovedV1 {
                    name: selector.name.clone(),
                },
            )))
            .await?;
        Ok(LevelResponse::LevelRemoved(selector.name.clone()))
    }
}
