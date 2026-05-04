use crate::*;

pub struct LevelService;

impl LevelService {
    pub async fn set(
        context: &ProjectLog,
        request: &SetLevel,
    ) -> Result<LevelResponse, LevelError> {
        let SetLevel::V1(set) = request;
        let level = Level::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();

        context
            .emit(LevelEvents::LevelSet(
                LevelSet::builder_v1().level(level.clone()).build().into(),
            ))
            .await?;

        Ok(LevelResponse::LevelSet(
            LevelSetResponse::builder_v1().level(level).build().into(),
        ))
    }

    pub async fn get(
        context: &ProjectLog,
        request: &GetLevel,
    ) -> Result<LevelResponse, LevelError> {
        let GetLevel::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let level = LevelRepo::new(context.scope()?)
            .fetch(&name)
            .await?
            .ok_or(LevelError::NotFound(name))?;
        Ok(LevelResponse::LevelDetails(
            LevelDetailsResponse::builder_v1()
                .level(level)
                .build()
                .into(),
        ))
    }

    pub async fn list(
        context: &ProjectLog,
        request: &ListLevels,
    ) -> Result<LevelResponse, LevelError> {
        let ListLevels::V1(listing) = request;
        let listed = LevelRepo::new(context.scope()?)
            .list(&listing.filters)
            .await?;
        if listed.total == 0 {
            Ok(LevelResponse::NoLevels)
        } else {
            Ok(LevelResponse::Levels(
                LevelsResponse::builder_v1()
                    .items(listed.items)
                    .total(listed.total)
                    .build()
                    .into(),
            ))
        }
    }

    pub async fn remove(
        context: &ProjectLog,
        request: &RemoveLevel,
    ) -> Result<LevelResponse, LevelError> {
        let RemoveLevel::V1(removal) = request;
        context
            .emit(LevelEvents::LevelRemoved(
                LevelRemoved::builder_v1()
                    .name(removal.name.clone())
                    .build()
                    .into(),
            ))
            .await?;
        Ok(LevelResponse::LevelRemoved(
            LevelRemovedResponse::builder_v1()
                .name(removal.name.clone())
                .build()
                .into(),
        ))
    }
}
