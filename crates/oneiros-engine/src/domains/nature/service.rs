use crate::*;

pub struct NatureService;

impl NatureService {
    pub async fn set(
        context: &ProjectContext,
        nature: Nature,
    ) -> Result<NatureResponse, NatureError> {
        let name = nature.name.clone();
        context.emit(NatureEvents::NatureSet(nature)).await?;
        Ok(NatureResponse::NatureSet(name))
    }

    pub fn get(context: &ProjectContext, name: &NatureName) -> Result<NatureResponse, NatureError> {
        let nature = NatureRepo::new(&context.db()?)
            .get(name)?
            .ok_or_else(|| NatureError::NotFound(name.clone()))?;
        Ok(NatureResponse::NatureDetails(nature))
    }

    pub fn list(context: &ProjectContext) -> Result<NatureResponse, NatureError> {
        let natures = NatureRepo::new(&context.db()?).list()?;
        if natures.is_empty() {
            Ok(NatureResponse::NoNatures)
        } else {
            Ok(NatureResponse::Natures(natures))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        name: &NatureName,
    ) -> Result<NatureResponse, NatureError> {
        context
            .emit(NatureEvents::NatureRemoved(NatureRemoved {
                name: name.clone(),
            }))
            .await?;
        Ok(NatureResponse::NatureRemoved(name.clone()))
    }
}
