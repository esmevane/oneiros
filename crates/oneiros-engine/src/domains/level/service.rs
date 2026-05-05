use crate::*;

pub struct LevelService;

impl LevelService {
    pub async fn set(
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &SetLevel,
    ) -> Result<LevelResponse, LevelError> {
        let SetLevel::V1(set) = request;
        let level = Level::builder()
            .name(set.name.clone())
            .description(set.description.clone())
            .prompt(set.prompt.clone())
            .build();
        let name = level.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Level(LevelEvents::LevelSet(
                LevelSet::builder_v1().level(level).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        let projected = LevelRepo::new(scope)
            .fetch(&name)
            .await?
            .ok_or(LevelError::NotFound(name))?;

        Ok(LevelResponse::LevelSet(
            LevelSetResponse::builder_v1()
                .level(projected)
                .build()
                .into(),
        ))
    }

    pub async fn get(
        scope: &Scope<AtBookmark>,
        request: &GetLevel,
    ) -> Result<LevelResponse, LevelError> {
        let GetLevel::V1(lookup) = request;
        let name = lookup.key.resolve()?;
        let level = LevelRepo::new(scope)
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
        scope: &Scope<AtBookmark>,
        request: &ListLevels,
    ) -> Result<LevelResponse, LevelError> {
        let ListLevels::V1(listing) = request;
        let listed = LevelRepo::new(scope).list(&listing.filters).await?;
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
        scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        request: &RemoveLevel,
    ) -> Result<LevelResponse, LevelError> {
        let RemoveLevel::V1(removal) = request;
        let name = removal.name.clone();

        let new_event = NewEvent::builder()
            .data(Events::Level(LevelEvents::LevelRemoved(
                LevelRemoved::builder_v1().name(name.clone()).build().into(),
            )))
            .build();
        mailbox.tell(Message::new(scope.clone(), new_event));

        scope
            .config()
            .fetch
            .until_absent(|| async { LevelRepo::new(scope).get(&name).await })
            .await?;

        Ok(LevelResponse::LevelRemoved(
            LevelRemovedResponse::builder_v1().name(name).build().into(),
        ))
    }
}
