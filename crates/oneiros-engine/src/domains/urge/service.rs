use crate::*;

pub struct UrgeService;

impl UrgeService {
    pub fn set(ctx: &ProjectContext, urge: Urge) -> Result<UrgeResponse, UrgeError> {
        let name = urge.name.clone();
        ctx.emit(UrgeEvents::UrgeSet(urge));
        Ok(UrgeResponse::UrgeSet(name))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<UrgeResponse, UrgeError> {
        let urge = ctx
            .with_db(|conn| UrgeRepo::new(conn).get(name))
            .map_err(UrgeError::Database)?
            .ok_or_else(|| UrgeError::NotFound(name.to_string()))?;
        Ok(UrgeResponse::UrgeDetails(urge))
    }

    pub fn list(ctx: &ProjectContext) -> Result<UrgeResponse, UrgeError> {
        let urges = ctx
            .with_db(|conn| UrgeRepo::new(conn).list())
            .map_err(UrgeError::Database)?;
        if urges.is_empty() {
            Ok(UrgeResponse::NoUrges)
        } else {
            Ok(UrgeResponse::Urges(urges))
        }
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<UrgeResponse, UrgeError> {
        let urge_name = UrgeName::new(name);
        ctx.emit(UrgeEvents::UrgeRemoved(UrgeRemoved {
            name: urge_name.clone(),
        }));
        Ok(UrgeResponse::UrgeRemoved(urge_name))
    }
}
