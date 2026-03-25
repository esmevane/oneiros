use crate::*;

pub struct PressureService;

impl PressureService {
    pub fn get(
        context: &ProjectContext,
        agent: &AgentName,
    ) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(&context.db()?).get(agent)?;
        Ok(PressureResponse::Readings(PressureResult {
            agent: agent.clone(),
            pressures,
        }))
    }

    pub fn list(context: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(&context.db()?).list()?;
        Ok(PressureResponse::Readings(PressureResult {
            agent: AgentName::new("all"),
            pressures,
        }))
    }
}
