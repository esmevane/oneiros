use crate::*;

pub struct SensationService;

impl SensationService {
    pub async fn set(
        context: &ProjectContext,
        SetSensation {
            name,
            description,
            prompt,
        }: &SetSensation,
    ) -> Result<SensationResponse, SensationError> {
        let sensation = Sensation::builder()
            .name(name.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();
        context
            .emit(SensationEvents::SensationSet(sensation))
            .await?;
        Ok(SensationResponse::SensationSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetSensation,
    ) -> Result<SensationResponse, SensationError> {
        let sensation = SensationRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| SensationError::NotFound(selector.name.clone()))?;
        Ok(SensationResponse::SensationDetails(sensation))
    }

    pub async fn list(
        context: &ProjectContext,
        ListSensations { filters }: &ListSensations,
    ) -> Result<SensationResponse, SensationError> {
        let listed = SensationRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(SensationResponse::NoSensations)
        } else {
            Ok(SensationResponse::Sensations(listed))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveSensation,
    ) -> Result<SensationResponse, SensationError> {
        context
            .emit(SensationEvents::SensationRemoved(SensationRemoved {
                name: selector.name.clone(),
            }))
            .await?;
        Ok(SensationResponse::SensationRemoved(selector.name.clone()))
    }
}
