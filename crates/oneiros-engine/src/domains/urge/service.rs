use crate::*;

pub struct UrgeService;

impl UrgeService {
    pub fn set(context: &ProjectContext, urge: Urge) -> Result<UrgeResponse, UrgeError> {
        let name = urge.name.clone();
        context.emit(UrgeEvents::UrgeSet(urge));
        Ok(UrgeResponse::UrgeSet(name))
    }

    pub fn get(context: &ProjectContext, name: &UrgeName) -> Result<UrgeResponse, UrgeError> {
        let urge = context
            .with_db(|conn| UrgeRepo::new(conn).get(name))?
            .ok_or_else(|| UrgeError::NotFound(name.clone()))?;
        Ok(UrgeResponse::UrgeDetails(urge))
    }

    pub fn list(context: &ProjectContext) -> Result<UrgeResponse, UrgeError> {
        let urges = context
            .with_db(|conn| UrgeRepo::new(conn).list())
            .map_err(UrgeError::Database)?;
        if urges.is_empty() {
            Ok(UrgeResponse::NoUrges)
        } else {
            Ok(UrgeResponse::Urges(urges))
        }
    }

    pub fn remove(context: &ProjectContext, name: &UrgeName) -> Result<UrgeResponse, UrgeError> {
        context.emit(UrgeEvents::UrgeRemoved(UrgeRemoved { name: name.clone() }));
        Ok(UrgeResponse::UrgeRemoved(name.clone()))
    }
}
