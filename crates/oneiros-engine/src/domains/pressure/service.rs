use crate::*;

pub struct PressureService;

impl PressureService {
    pub fn get(ctx: &ProjectContext, agent: &str) -> Result<PressureResponse, PressureError> {
        let pressures = ctx
            .with_db(|conn| PressureRepo::new(conn).get(agent))
            .map_err(PressureError::Database)?;
        Ok(PressureResponse::Readings(pressures))
    }

    pub fn list(ctx: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = ctx
            .with_db(|conn| PressureRepo::new(conn).list())
            .map_err(PressureError::Database)?;
        Ok(PressureResponse::Readings(pressures))
    }
}
