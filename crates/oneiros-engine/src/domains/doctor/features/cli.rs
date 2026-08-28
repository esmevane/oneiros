use crate::*;

pub(crate) struct DoctorCli;

impl DoctorCli {
    pub(crate) async fn execute(config: &Config) -> Result<Rendered<Responses>, DoctorError> {
        // TODO: thread Databases from server when CLI dispatch is refactored.
        let databases = Databases::new(config.clone());
        let response = DoctorService::check(config, &databases).await;

        Ok(DoctorView::new(response).render().map(Into::into))
    }
}
