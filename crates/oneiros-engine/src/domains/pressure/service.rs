use crate::*;

pub(crate) struct PressureService;

impl PressureService {
    pub(crate) async fn get(
        context: &ProjectContext,
        selector: &GetPressure,
    ) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(context).get(&selector.agent).await?;
        Ok(PressureResponse::Readings(PressureResult {
            agent: selector.agent.clone(),
            pressures,
        }))
    }

    pub(crate) async fn list(context: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(context).list().await?;
        Ok(PressureResponse::AllReadings(ListPressureResult {
            pressures,
        }))
    }
}
