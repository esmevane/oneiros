use crate::*;

pub struct DoctorCli;

impl DoctorCli {
    pub fn execute(ctx: &SystemContext) -> Result<Rendered<Responses>, Box<dyn std::error::Error>> {
        let response = DoctorService::check(ctx);

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
