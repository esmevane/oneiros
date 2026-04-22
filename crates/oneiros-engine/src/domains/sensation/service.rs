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
        let name = selector.key.resolve()?;
        let sensation = SensationRepo::new(context)
            .get(&name)
            .await?
            .ok_or(SensationError::NotFound(name))?;
        let ref_token = RefToken::new(Ref::sensation(sensation.name.clone()));
        Ok(SensationResponse::SensationDetails(
            Response::new(sensation).with_ref_token(ref_token),
        ))
    }

    pub async fn list(
        context: &ProjectContext,
        ListSensations { filters }: &ListSensations,
    ) -> Result<SensationResponse, SensationError> {
        let listed = SensationRepo::new(context).list(filters).await?;
        if listed.total == 0 {
            Ok(SensationResponse::NoSensations)
        } else {
            Ok(SensationResponse::Sensations(listed.map(|e| {
                let ref_token = RefToken::new(Ref::sensation(e.name.clone()));
                Response::new(e).with_ref_token(ref_token)
            })))
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
