use crate::*;

pub(crate) struct SetupService;

impl SetupService {
    pub(crate) async fn run(
        config: &Config,
        request: &SetupRequest,
    ) -> Result<SetupResponse, SetupError> {
        let details = request.current()?;
        let mut steps = Vec::new();

        // 1. Service install — gated by `install_host`.
        let server_ready = matches!(
            HostService::status(config).await,
            HostResponse::ServiceRunning(_)
        );

        if !server_ready {
            if !details.install_host {
                steps.push(SetupStep::ServiceSkipped);
                return Ok(SetupResponse::SetupComplete(
                    SetupCompleteResponse::builder_v1()
                        .steps(steps)
                        .build()
                        .into(),
                ));
            }

            match HostService::install(config) {
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

            match HostService::start(config).await {
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

        // 2. Host init (always, idempotent).
        let host_secret = HostKey::new(config.platform()).load()?;
        let host_client = match host_secret {
            Some(secret) => {
                let host_token = HostToken::generate(&secret);
                Client::with_bearer(config.base_url(), &host_token.to_string())
            }
            None => Ok(Client::new(config.base_url())),
        }?;
        let host_request: InitHost = InitHost::builder_v1()
            .maybe_name(details.name.clone())
            .yes(true)
            .build()
            .into();

        let host_init_result: Result<HostResponse, ClientError> =
            match host_request.execute_request(&host_client).await {
                Ok(bytes) => serde_json::from_slice(&bytes)
                    .map_err(|e| ClientError::InvalidRequest(format!("host init response: {e}"))),
                Err(e) => Err(e),
            };
        match host_init_result {
            Ok(HostResponse::HostInitialized(_)) => {
                steps.push(SetupStep::HostInitialized);
            }
            Ok(HostResponse::HostAlreadyInitialized) => {
                steps.push(SetupStep::HostAlreadyInitialized);
            }
            Ok(unexpected) => {
                steps.push(SetupStep::StepFailed {
                    step: "host init".into(),
                    reason: format!("unexpected host init response: {unexpected}"),
                });
                return Ok(SetupResponse::SetupComplete(
                    SetupCompleteResponse::builder_v1()
                        .steps(steps)
                        .build()
                        .into(),
                ));
            }
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "host init".into(),
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

        // 3. Project create (always, idempotent).
        let project_request: CreateProject = CreateProject::builder_v1()
            .name(config.project.clone())
            .yes(true)
            .build()
            .into();
        let project_create_result: Result<ProjectResponse, ClientError> =
            match project_request.execute_request(&host_client).await {
                Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| {
                    ClientError::InvalidRequest(format!("project create response: {e}"))
                }),
                Err(e) => Err(e),
            };
        let project_token: Option<Token> = match project_create_result {
            Ok(ProjectResponse::Created(ProjectCreatedResponse::V1(result))) => {
                steps.push(SetupStep::ProjectInitialized(result.project.name));
                Some(result.token)
            }
            Ok(ProjectResponse::ProjectAlreadyExists(ProjectAlreadyExistsResponse::V1(
                details,
            ))) => {
                steps.push(SetupStep::ProjectAlreadyExists(details.project_name));
                None
            }
            Ok(_) => None,
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "project create".into(),
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

        // 4. Seed core (always, idempotent).
        match project_client.post("/seed/core", &()).await {
            Ok(_) => steps.push(SetupStep::VocabularySeeded),
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed core".into(),
                    reason: e.to_string(),
                });
            }
        }

        // 5. Seed agents (always, idempotent).
        match project_client.post("/seed/agents", &()).await {
            Ok(_) => steps.push(SetupStep::AgentsSeeded),
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed agents".into(),
                    reason: e.to_string(),
                });
            }
        }

        // 6. MCP config — gated by `init_mcp`.
        if details.init_mcp {
            let mcp_request: InitMcp = InitMcp::builder_v1().yes(true).build().into();
            match McpConfigService::init(config, &mcp_request) {
                Ok(McpResponses::McpConfigWritten(_)) => {
                    steps.push(SetupStep::McpConfigured);
                }
                Ok(McpResponses::McpConfigExists(_)) => {
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
