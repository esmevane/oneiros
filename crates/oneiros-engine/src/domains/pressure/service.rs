use crate::*;

pub struct PressureService;

impl PressureService {
    pub async fn get(
        context: &ProjectContext,
        selector: &GetPressure,
    ) -> Result<PressureResponse, PressureError> {
        let details = selector.current()?;
        let pressures = PressureRepo::new(context).get(&details.agent).await?;
        Ok(PressureResponse::Readings(
            ReadingsResponse::builder_v1()
                .agent(details.agent)
                .pressures(pressures)
                .build()
                .into(),
        ))
    }

    pub async fn list(context: &ProjectContext) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(context).list().await?;
        Ok(PressureResponse::AllReadings(
            AllReadingsResponse::builder_v1()
                .pressures(pressures)
                .build()
                .into(),
        ))
    }
}
