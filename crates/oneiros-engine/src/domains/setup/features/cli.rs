use crate::*;

pub(crate) struct SetupCli;

impl SetupCli {
    pub(crate) async fn execute(
        config: &Config,
        request: &SetupRequest,
    ) -> Result<Rendered<Responses>, SetupError> {
        let response = SetupService::run(config, request).await?;

        Ok(SetupView::new(response).render().map(Into::into))
    }
}
