use crate::*;

pub struct NatureService;

impl NatureService {
    pub async fn set(
        context: &ProjectContext,
        SetNature {
            name,
            description,
            prompt,
        }: &SetNature,
    ) -> Result<NatureResponse, NatureError> {
        let nature = Nature::builder()
            .name(name.clone())
            .description(description.clone())
            .prompt(prompt.clone())
            .build();
        context.emit(NatureEvents::NatureSet(nature)).await?;
        Ok(NatureResponse::NatureSet(name.clone()))
    }

    pub async fn get(
        context: &ProjectContext,
        selector: &GetNature,
    ) -> Result<NatureResponse, NatureError> {
        let nature = NatureRepo::new(context)
            .get(&selector.name)
            .await?
            .ok_or_else(|| NatureError::NotFound(selector.name.clone()))?;
        Ok(NatureResponse::NatureDetails(nature))
    }

    pub async fn list(context: &ProjectContext) -> Result<NatureResponse, NatureError> {
        let natures = NatureRepo::new(context).list().await?;
        if natures.is_empty() {
            Ok(NatureResponse::NoNatures)
        } else {
            Ok(NatureResponse::Natures(natures))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        selector: &RemoveNature,
    ) -> Result<NatureResponse, NatureError> {
        context
            .emit(NatureEvents::NatureRemoved(NatureRemoved {
                name: selector.name.clone(),
            }))
            .await?;
        Ok(NatureResponse::NatureRemoved(selector.name.clone()))
    }
}
