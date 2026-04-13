use crate::*;

pub(crate) struct UrgeService;

impl UrgeService {
    pub(crate) async fn set(
        context: &ProjectContext,
        SetUrge {
            name,
            description,
            prompt,
        }: &SetUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let urge = Urge::builder()
            .name(name.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();
        context.emit(UrgeEvents::UrgeSet(urge)).await?;
        Ok(UrgeResponse::UrgeSet(name.clone()))
    }

    pub(crate) async fn get(
        context: &ProjectContext,
        selector: &GetUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        let urge = UrgeRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| UrgeError::NotFound(selector.name.clone()))?;
        Ok(UrgeResponse::UrgeDetails(urge))
    }

    pub(crate) async fn list(
        context: &ProjectContext,
        ListUrges { filters }: &ListUrges,
    ) -> Result<UrgeResponse, UrgeError> {
        let listed = UrgeRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(UrgeResponse::NoUrges)
        } else {
            Ok(UrgeResponse::Urges(listed))
        }
    }

    pub(crate) async fn remove(
        context: &ProjectContext,
        selector: &RemoveUrge,
    ) -> Result<UrgeResponse, UrgeError> {
        context
            .emit(UrgeEvents::UrgeRemoved(UrgeRemoved {
                name: selector.name.clone(),
            }))
            .await?;
        Ok(UrgeResponse::UrgeRemoved(selector.name.clone()))
    }
}
