use crate::*;

pub(crate) struct SetupService;

impl SetupService {
    pub(crate) async fn run(config: &Config, request: &SetupRequest) -> Result<SetupResponse, SetupError> {
        let mut steps = Vec::new();

        // 1. System init (always, idempotent)
        let system_ctx = config.system();
        let system_request = InitSystem {
            name: request.name.clone(),
            yes: true,
        };

        match SystemService::init(&system_ctx, &system_request).await {
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
                return Ok(SetupResponse::SetupComplete(steps));
            }
        }

        // 2. Project init (always, idempotent)
        let project_request = InitProject::builder().yes(true).build();

        match ProjectService::init(&system_ctx, &project_request).await {
            Ok(ProjectResponse::Initialized(result)) => {
                steps.push(SetupStep::ProjectInitialized(result.brain_name));
            }
            Ok(ProjectResponse::BrainAlreadyExists(name)) => {
                steps.push(SetupStep::ProjectAlreadyExists(name));
            }
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "project init".into(),
                    reason: e.to_string(),
                });
                return Ok(SetupResponse::SetupComplete(steps));
            }
            _ => {}
        }

        // 3. Seed core (always, idempotent) — routes through HTTP client
        let client = config.client();

        match SeedService::core(&client).await {
            Ok(_) => steps.push(SetupStep::VocabularySeeded),
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed core".into(),
                    reason: e.to_string(),
                });
            }
        }

        // 4. Seed agents (always, idempotent)
        match SeedService::agents(&client).await {
            Ok(_) => steps.push(SetupStep::AgentsSeeded),
            Err(e) => {
                steps.push(SetupStep::StepFailed {
                    step: "seed agents".into(),
                    reason: e.to_string(),
                });
            }
        }

        // 5. MCP config (prompt unless --yes)
        let do_mcp = request.yes
            || inquire::Confirm::new("Set up MCP config for Claude Code?")
                .with_default(true)
                .prompt()
                .unwrap_or(false);

        if do_mcp {
            let mcp_request = InitMcp::builder().yes(true).build();
            match McpConfigService::init(config, &mcp_request) {
                Ok(McpConfigResponse::McpConfigWritten(_)) => {
                    steps.push(SetupStep::McpConfigured);
                }
                Ok(McpConfigResponse::McpConfigExists(_)) => {
                    // With yes=true this shouldn't happen, but handle it
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

        // 6. Service install + start (prompt unless --yes)
        let do_service = request.yes
            || inquire::Confirm::new("Install and start the oneiros service?")
                .with_default(true)
                .prompt()
                .unwrap_or(false);

        if do_service {
            match ServiceService::install(config) {
                Ok(_) => steps.push(SetupStep::ServiceInstalled),
                Err(e) => {
                    steps.push(SetupStep::StepFailed {
                        step: "service install".into(),
                        reason: e.to_string(),
                    });
                }
            }

            match ServiceService::start(config).await {
                Ok(_) => steps.push(SetupStep::ServiceStarted),
                Err(e) => {
                    steps.push(SetupStep::StepFailed {
                        step: "service start".into(),
                        reason: e.to_string(),
                    });
                }
            }
        } else {
            steps.push(SetupStep::ServiceSkipped);
        }

        Ok(SetupResponse::SetupComplete(steps))
    }
}
