use crate::*;

pub struct DoctorCli;

impl DoctorCli {
    pub async fn execute(config: &Config) -> Result<Rendered<Responses>, DoctorError> {
        let response = DoctorService::check(config).await;

        let prompt = match &response {
            DoctorResponse::CheckupStatus(checks) => {
                let mut lines = vec!["Oneiros health check:".to_string()];

                for check in checks {
                    let line = match check {
                        DoctorCheck::Initialized => "  [+] System initialized".into(),
                        DoctorCheck::NotInitialized => {
                            "  [!] System not initialized — run `oneiros setup`".into()
                        }
                        DoctorCheck::DatabaseOk(label) => format!("  [+] Database OK ({label})"),
                        DoctorCheck::EventLogReady(count) => {
                            format!("  [+] Event log ready ({count} events)")
                        }
                        DoctorCheck::BrainExists(name) => format!("  [+] Brain '{name}' exists"),
                        DoctorCheck::BrainMissing(name) => {
                            format!("  [!] Brain '{name}' not found — run `oneiros project init`")
                        }
                        DoctorCheck::VocabularySeeded => "  [+] Vocabulary seeded".into(),
                        DoctorCheck::VocabularyMissing => {
                            "  [!] Vocabulary missing — run `oneiros seed core`".into()
                        }
                        DoctorCheck::AgentsSeeded => "  [+] Canonical agents present".into(),
                        DoctorCheck::AgentsMissing => {
                            "  [!] Canonical agents missing — run `oneiros seed agents`".into()
                        }
                        DoctorCheck::McpConfigured => "  [+] MCP config present".into(),
                        DoctorCheck::McpMissing => {
                            "  [-] MCP config missing — run `oneiros mcp init`".into()
                        }
                        DoctorCheck::ServiceRunning => "  [+] Service running".into(),
                        DoctorCheck::ServiceStopped => {
                            "  [-] Service stopped — run `oneiros service start`".into()
                        }
                        DoctorCheck::ServiceNotInstalled => {
                            "  [-] Service not installed — run `oneiros service install`".into()
                        }
                    };
                    lines.push(line);
                }

                lines.join("\n")
            }
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
