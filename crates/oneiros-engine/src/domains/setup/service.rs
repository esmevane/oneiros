use crate::*;

pub struct SetupService;

impl SetupService {
    pub async fn run(config: &Config, request: &SetupRequest) -> Result<SetupResponse, SetupError> {
        let details = request.current()?;
        let mut steps = Vec::new();

        // 1. Server reachability is the precondition. Setup talks to a running
        //    server; if there isn't one, offer to install and start it.
        let server_ready = matches!(
            ServiceService::status(config).await,
            ServiceResponse::ServiceRunning(_)
        );

        if !server_ready {
            let do_service = details.yes
                || inquire::Confirm::new(
                    "The oneiros service isn't running. Install and start it now?",
                )
                .with_default(true)
                .prompt()
                .unwrap_or(false);

            if !do_service {
                steps.push(SetupStep::ServiceSkipped);
                return Ok(SetupResponse::SetupComplete(
                    SetupCompleteResponse::builder_v1()
                        .steps(steps)
                        .build()
                        .into(),
                ));
            }

            match ServiceService::install(config) {
                Ok(_) => steps.push(SetupStep::ServiceInstalled),
                Err(e) => {
                    steps.push(SetupStep::StepFailed {
                        step: "service install".into(),
                        reason: e.to_string(),
                    });
                    return Ok(SetupResponse::SetupComplete(
                        SetupCompleteResponse::builder_v1()
                            .steps(steps)
                            .build()
                            .into(),
                    ));
                }
            }

            match ServiceService::start(config).await {
                Ok(_) => steps.push(SetupStep::ServiceStarted),
                Err(e) => {
                    steps.push(SetupStep::StepFailed {
                        step: "service start".into(),
                        reason: e.to_string(),
                    });
                    return Ok(SetupResponse::SetupComplete(
                        SetupCompleteResponse::builder_v1()
                            .steps(steps)
                            .build()
                            .into(),
                    ));
                }
            }
        }

        // 2. System init (always, idempotent) — over HTTP.
        let host_client = Client::new(config.base_url());
        let system_request: InitSystem = InitSystem::builder_v1()
            .maybe_name(details.name.clone())
            .yes(true)
            .build()
            .into();

        match SystemClient::new(&host_client).init(&system_request).await {
            Ok(SystemResponse::SystemInitialized(_)) => {
                steps.push(SetupStep::SystemInitialized);
            }
            Ok(SystemResponse::HostAlreadyInitialized) => {
                steps.push(SetupStep::SystemAlreadyInitialized);
            }
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "system init".into(),
                    reason: e.to_string(),
                });
                return Ok(SetupResponse::SetupComplete(
                    SetupCompleteResponse::builder_v1()
                        .steps(steps)
                        .build()
                        .into(),
                ));
            }
        }

        // 3. Project init (always, idempotent) — over HTTP. Capture the token
        //    from the response so the seed calls can authenticate.
        let project_request: InitProject = InitProject::builder_v1().yes(true).build().into();
        let project_token: Option<Token> = match ProjectClient::new(&host_client)
            .init(&project_request)
            .await
        {
            Ok(ProjectResponse::Initialized(InitializedResponse::V1(result))) => {
                steps.push(SetupStep::ProjectInitialized(result.brain_name));
                Some(result.token)
            }
            Ok(ProjectResponse::BrainAlreadyExists(BrainAlreadyExistsResponse::V1(details))) => {
                steps.push(SetupStep::ProjectAlreadyExists(details.brain_name));
                None
            }
            Ok(_) => None,
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "project init".into(),
                    reason: e.to_string(),
                });
                return Ok(SetupResponse::SetupComplete(
                    SetupCompleteResponse::builder_v1()
                        .steps(steps)
                        .build()
                        .into(),
                ));
            }
        };

        // Resolve the token for the seed calls: prefer a freshly-issued one,
        // fall back to whatever's already on disk (for repeat runs).
        let token = project_token.or_else(|| config.token());

        let Some(token) = token else {
            steps.push(SetupStep::StepFailed {
                step: "seed".into(),
                reason: "no project token available — cannot authenticate seed calls".into(),
            });
            return Ok(SetupResponse::SetupComplete(
                SetupCompleteResponse::builder_v1()
                    .steps(steps)
                    .build()
                    .into(),
            ));
        };

        let project_client = match Client::with_token(config.base_url(), token) {
            Ok(client) => client,
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed".into(),
                    reason: e.to_string(),
                });
                return Ok(SetupResponse::SetupComplete(
                    SetupCompleteResponse::builder_v1()
                        .steps(steps)
                        .build()
                        .into(),
                ));
            }
        };

        let seed = SeedClient::new(&project_client);

        // 4. Seed core (always, idempotent) — over HTTP.
        match seed.core().await {
            Ok(_) => steps.push(SetupStep::VocabularySeeded),
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed core".into(),
                    reason: e.to_string(),
                });
            }
        }

        // 5. Seed agents (always, idempotent) — over HTTP.
        match seed.agents().await {
            Ok(_) => steps.push(SetupStep::AgentsSeeded),
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed agents".into(),
                    reason: e.to_string(),
                });
            }
        }

        // 6. MCP config (prompt unless --yes) — local file write.
        let do_mcp = details.yes
            || inquire::Confirm::new("Set up MCP config for Claude Code?")
                .with_default(true)
                .prompt()
                .unwrap_or(false);

        if do_mcp {
            let mcp_request: InitMcp = InitMcp::builder_v1().yes(true).build().into();
            match McpConfigService::init(config, &mcp_request) {
                Ok(McpConfigResponse::McpConfigWritten(_)) => {
                    steps.push(SetupStep::McpConfigured);
                }
                Ok(McpConfigResponse::McpConfigExists(_)) => {
                    steps.push(SetupStep::McpConfigured);
                }
                Err(e) => {
                    steps.push(SetupStep::StepFailed {
                        step: "mcp init".into(),
                        reason: e.to_string(),
                    });
                }
            }
        } else {
            steps.push(SetupStep::McpSkipped);
        }

        Ok(SetupResponse::SetupComplete(
            SetupCompleteResponse::builder_v1()
                .steps(steps)
                .build()
                .into(),
        ))
    }
}
