use crate::*;

pub(crate) struct DoctorCli;

impl DoctorCli {
    pub(crate) async fn execute(config: &Config) -> Result<Rendered<Responses>, DoctorError> {
        let response = DoctorService::check(config).await;

        let prompt = match &response {
            DoctorResponse::CheckupStatus(checks) => DoctorView::checklist(checks),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
