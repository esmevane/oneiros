use crate::*;

pub(crate) struct PressureService;

impl PressureService {
    #[expect(deprecated)]
    pub(crate) async fn get(
        context: &ProjectLog,
        selector: &GetPressure,
    ) -> Result<PressureResponse, PressureError> {
        let details = selector.current()?;
        let pressures = PressureRepo::new(context.scope().await?)
            .get(&details.agent)
            .await?;
        Ok(PressureResponse::Readings(
            ReadingsResponse::builder_v1()
                .agent(details.agent)
                .pressures(pressures)
                .build()
                .into(),
        ))
    }

    #[expect(deprecated)]
    pub(crate) async fn list(context: &ProjectLog) -> Result<PressureResponse, PressureError> {
        let pressures = PressureRepo::new(context.scope().await?).list().await?;
        Ok(PressureResponse::AllReadings(
            AllReadingsResponse::builder_v1()
                .pressures(pressures)
                .build()
                .into(),
        ))
    }
}
