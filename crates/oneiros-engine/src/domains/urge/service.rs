use crate::contexts::ProjectContext;

use super::errors::UrgeError;
use super::events::{UrgeEvents, UrgeRemoved};
use super::model::Urge;
use super::repo::UrgeRepo;
use super::responses::UrgeResponse;

pub struct UrgeService;

impl UrgeService {
    pub fn set(ctx: &ProjectContext, urge: Urge) -> Result<UrgeResponse, UrgeError> {
        ctx.emit(UrgeEvents::UrgeSet(urge.clone()));
        Ok(UrgeResponse::Set(urge))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<UrgeResponse, UrgeError> {
        let urge = ctx
            .with_db(|conn| UrgeRepo::new(conn).get(name))
            .map_err(UrgeError::Database)?
            .ok_or_else(|| UrgeError::NotFound(name.to_string()))?;
        Ok(UrgeResponse::Found(urge))
    }

    pub fn list(ctx: &ProjectContext) -> Result<UrgeResponse, UrgeError> {
        let urges = ctx
            .with_db(|conn| UrgeRepo::new(conn).list())
            .map_err(UrgeError::Database)?;
        Ok(UrgeResponse::Listed(urges))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<UrgeResponse, UrgeError> {
        ctx.emit(UrgeEvents::UrgeRemoved(UrgeRemoved {
            name: name.to_string(),
        }));
        Ok(UrgeResponse::Removed)
    }
}
