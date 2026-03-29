use crate::*;

pub struct UrgeService;

impl UrgeService {
    pub async fn set(context: &ProjectContext, urge: Urge) -> Result<UrgeResponse, UrgeError> {
        let name = urge.name.clone();
        context.emit(UrgeEvents::UrgeSet(urge)).await?;
        Ok(UrgeResponse::UrgeSet(name))
    }

    pub fn get(context: &ProjectContext, name: &UrgeName) -> Result<UrgeResponse, UrgeError> {
        let urge = UrgeRepo::new(&context.db()?)
            .get(name)?
            .ok_or_else(|| UrgeError::NotFound(name.clone()))?;
        Ok(UrgeResponse::UrgeDetails(urge))
    }

    pub fn list(context: &ProjectContext) -> Result<UrgeResponse, UrgeError> {
        let urges = UrgeRepo::new(&context.db()?).list()?;
        if urges.is_empty() {
            Ok(UrgeResponse::NoUrges)
        } else {
            Ok(UrgeResponse::Urges(urges))
        }
    }

    pub async fn remove(
        context: &ProjectContext,
        name: &UrgeName,
    ) -> Result<UrgeResponse, UrgeError> {
        context
            .emit(UrgeEvents::UrgeRemoved(UrgeRemoved { name: name.clone() }))
            .await?;
        Ok(UrgeResponse::UrgeRemoved(name.clone()))
    }
}
