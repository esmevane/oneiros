use crate::*;

pub struct SetupCli;

impl SetupCli {
    pub async fn execute(
        config: &Config,
        request: &SetupRequest,
    ) -> Result<Rendered<Responses>, SetupError> {
        let response = SetupService::run(config, request).await?;

        let prompt = match &response {
            SetupResponse::SetupComplete(steps) => {
                let mut lines = vec!["Setup complete.".to_string()];

                for step in steps {
                    let line = match step {
                        SetupStep::SystemInitialized => "  [+] System initialized".into(),
                        SetupStep::SystemAlreadyInitialized => {
                            "  [=] System already initialized".into()
                        }
                        SetupStep::ProjectInitialized(name) => {
                            format!("  [+] Brain '{name}' created")
                        }
                        SetupStep::ProjectAlreadyExists(name) => {
                            format!("  [=] Brain '{name}' already exists")
                        }
                        SetupStep::VocabularySeeded => "  [+] Vocabulary seeded".into(),
                        SetupStep::AgentsSeeded => "  [+] Agents seeded".into(),
                        SetupStep::McpConfigured => "  [+] MCP config written".into(),
                        SetupStep::McpSkipped => "  [-] MCP config skipped".into(),
                        SetupStep::ServiceInstalled => "  [+] Service installed".into(),
                        SetupStep::ServiceStarted => "  [+] Service started".into(),
                        SetupStep::ServiceSkipped => "  [-] Service skipped".into(),
                        SetupStep::StepFailed { step, reason } => {
                            format!("  [!] {step} failed: {reason}")
                        }
                    };
                    lines.push(line);
                }

                lines.join("\n")
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
