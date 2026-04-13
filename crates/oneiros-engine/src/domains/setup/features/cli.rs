use crate::*;

pub(crate) struct SetupCli;

impl SetupCli {
    pub(crate) async fn execute(
        config: &Config,
        request: &SetupRequest,
    ) -> Result<Rendered<Responses>, SetupError> {
        let response = SetupService::run(config, request).await?;

        let prompt = match &response {
            SetupResponse::SetupComplete(steps) => SetupView::steps(steps),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
