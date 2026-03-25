use crate::*;

pub struct PressureService;

impl PressureService {
    pub fn get(
        context: &ProjectContext,
        agent: &AgentName,
    ) -> Result<PressureResponse, PressureError> {
        let pressures = context
            .with_db(|conn| PressureRepo::new(conn).get(agent))
            .map_err(PressureError::Database)?;
        Ok(PressureResponse::Readings(PressureResult {
            agent: agent.clone(),
            pressures,
        }))
    }

    pub fn list(context: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = context
            .with_db(|conn| PressureRepo::new(conn).list())
            .map_err(PressureError::Database)?;
        Ok(PressureResponse::Readings(PressureResult {
            agent: AgentName::new("all"),
            pressures,
        }))
    }
}
