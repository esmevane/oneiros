use crate::*;

pub struct SetupCli;

impl SetupCli {
    pub async fn execute(
        config: &Config,
        request: &SetupRequest,
    ) -> Result<Rendered<Responses>, SetupError> {
        let response = SetupService::run(config, request).await?;

        Ok(SetupView::new(response).render().map(Into::into))
    }
}
