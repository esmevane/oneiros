use crate::*;

pub struct DoctorCli;

impl DoctorCli {
    pub async fn execute(context: &SystemContext) -> Result<Rendered<Responses>, DoctorError> {
        let response = DoctorService::check(context);

        let prompt = match &response {
            DoctorResponse::CheckupStatus(checks) => checks
                .iter()
                .map(|c| format!("{c:?}"))
                .collect::<Vec<_>>()
                .join("\n"),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
