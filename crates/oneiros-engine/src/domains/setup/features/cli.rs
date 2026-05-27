use crate::*;

pub(crate) struct SetupCli;

impl SetupCli {
    pub(crate) async fn execute(
        config: &Config,
        request: &SetupRequest,
    ) -> Result<Rendered<Responses>, SetupError> {
        let request = Self::resolve(config, request).await;
        let response = SetupService::run(config, &request).await?;

        Ok(SetupView::new(response).render().map(Into::into))
    }

    async fn resolve(config: &Config, request: &SetupRequest) -> SetupRequest {
        let SetupRequest::V1(details) = request;

        let mut install_host = details.install_host || details.accept_all;
        let mut init_mcp = details.init_mcp || details.accept_all;

        if install_host && init_mcp {
            return SetupRequest::V1(SetupRequestV1 {
                name: details.name.clone(),
                install_host,
                init_mcp,
                accept_all: false,
            });
        }

        if !install_host {
            let server_ready = matches!(
                HostService::status(config).await,
                HostResponse::ServiceRunning(_)
            );

            if !server_ready {
                install_host = inquire::Confirm::new(
                    "The oneiros service isn't running. Install and start it now?",
                )
                .with_default(true)
                .prompt()
                .unwrap_or(false);
            }
        }

        if !init_mcp {
            init_mcp = inquire::Confirm::new("Set up MCP config for Claude Code?")
                .with_default(true)
                .prompt()
                .unwrap_or(false);
        }

        SetupRequest::V1(SetupRequestV1 {
            name: details.name.clone(),
            install_host,
            init_mcp,
            accept_all: false,
        })
    }
}
