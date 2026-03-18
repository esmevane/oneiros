use crate::contexts::ProjectContext;

use super::errors::PressureError;
use super::repo::PressureRepo;
use super::responses::PressureResponse;

pub struct PressureService;

impl PressureService {
    pub fn get(ctx: &ProjectContext, agent: &str) -> Result<PressureResponse, PressureError> {
        let pressures = ctx
            .with_db(|conn| PressureRepo::new(conn).get(agent))
            .map_err(PressureError::Database)?;
        Ok(PressureResponse::Found(pressures))
    }

    pub fn list(ctx: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = ctx
            .with_db(|conn| PressureRepo::new(conn).list())
            .map_err(PressureError::Database)?;
        Ok(PressureResponse::Listed(pressures))
    }
}
