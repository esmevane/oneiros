use crate::*;

pub struct DoctorCli;

impl DoctorCli {
    pub async fn execute(config: &Config) -> Result<Rendered<Responses>, DoctorError> {
        let response = DoctorService::check(config).await;

        Ok(DoctorView::new(response).render().map(Into::into))
    }
}
