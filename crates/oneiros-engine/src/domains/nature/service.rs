use crate::contexts::ProjectContext;

use super::errors::NatureError;
use super::events::{NatureEvents, NatureRemoved};
use super::model::Nature;
use super::repo::NatureRepo;
use super::responses::NatureResponse;

pub struct NatureService;

impl NatureService {
    pub fn set(ctx: &ProjectContext, nature: Nature) -> Result<NatureResponse, NatureError> {
        ctx.emit(NatureEvents::NatureSet(nature.clone()));
        Ok(NatureResponse::Set(nature))
    }

    pub fn get(ctx: &ProjectContext, name: &str) -> Result<NatureResponse, NatureError> {
        let nature = ctx
            .with_db(|conn| NatureRepo::new(conn).get(name))
            .map_err(NatureError::Database)?
            .ok_or_else(|| NatureError::NotFound(name.to_string()))?;
        Ok(NatureResponse::Found(nature))
    }

    pub fn list(ctx: &ProjectContext) -> Result<NatureResponse, NatureError> {
        let natures = ctx
            .with_db(|conn| NatureRepo::new(conn).list())
            .map_err(NatureError::Database)?;
        Ok(NatureResponse::Listed(natures))
    }

    pub fn remove(ctx: &ProjectContext, name: &str) -> Result<NatureResponse, NatureError> {
        ctx.emit(NatureEvents::NatureRemoved(NatureRemoved {
            name: name.to_string(),
        }));
        Ok(NatureResponse::Removed)
    }
}
