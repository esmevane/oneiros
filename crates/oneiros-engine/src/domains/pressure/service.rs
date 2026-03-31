use crate::*;

pub struct PressureService;

impl PressureService {
    pub async fn get(
        context: &ProjectContext,
        selector: &GetPressure,
    ) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(context).get(&selector.agent).await?;
        Ok(PressureResponse::Readings(PressureResult {
            agent: selector.agent.clone(),
            pressures,
        }))
    }

    pub async fn list(context: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(context).list().await?;
        Ok(PressureResponse::AllReadings(ListPressureResult {
            pressures,
        }))
    }
}
