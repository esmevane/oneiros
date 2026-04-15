//! Doctor view — presentation for health check results.

use crate::*;

pub struct DoctorView {
    response: DoctorResponse,
}

impl DoctorView {
    pub fn new(response: DoctorResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<DoctorResponse> {
        match self.response {
            DoctorResponse::CheckupStatus(checks) => {
                let prompt = Self::checklist(&checks);
                Rendered::new(DoctorResponse::CheckupStatus(checks), prompt, String::new())
            }
        }
    }

    fn checklist(checks: &[DoctorCheck]) -> String {
        let mut lines = vec![format!("{}", "Oneiros health check:".heading())];

        for check in checks {
            let line = match check {
                DoctorCheck::Initialized => {
                    format!("  {} System initialized", "✓".success())
                }
                DoctorCheck::NotInitialized => {
                    format!(
                        "  {} System not initialized {} run `oneiros setup`",
                        "!".warning(),
                        "—".muted()
                    )
                }
                DoctorCheck::DatabaseOk(label) => {
                    format!("  {} Database OK ({})", "✓".success(), label)
                }
                DoctorCheck::EventLogReady(count) => {
                    format!("  {} Event log ready ({} events)", "✓".success(), count)
                }
                DoctorCheck::BrainExists(name) => {
                    format!("  {} Brain '{}' exists", "✓".success(), name)
                }
                DoctorCheck::BrainMissing(name) => {
                    format!(
                        "  {} Brain '{}' not found {} run `oneiros project init`",
                        "!".warning(),
                        name,
                        "—".muted()
                    )
                }
                DoctorCheck::VocabularySeeded => {
                    format!("  {} Vocabulary seeded", "✓".success())
                }
                DoctorCheck::VocabularyMissing => {
                    format!(
                        "  {} Vocabulary missing {} run `oneiros seed core`",
                        "!".warning(),
                        "—".muted()
                    )
                }
                DoctorCheck::AgentsSeeded => {
                    format!("  {} Canonical agents present", "✓".success())
                }
                DoctorCheck::AgentsMissing => {
                    format!(
                        "  {} Canonical agents missing {} run `oneiros seed agents`",
                        "!".warning(),
                        "—".muted()
                    )
                }
                DoctorCheck::HostKeyOk => {
                    format!("  {} Host keypair present", "✓".success())
                }
                DoctorCheck::HostKeyMissing => {
                    format!(
                        "  {} Host keypair missing {} run `oneiros system init`",
                        "!".warning(),
                        "—".muted()
                    )
                }
                DoctorCheck::McpConfigured => {
                    format!("  {} MCP config present", "✓".success())
                }
                DoctorCheck::McpMissing => {
                    format!(
                        "  {} MCP config missing {} run `oneiros mcp init`",
                        "−".muted(),
                        "—".muted()
                    )
                }
                DoctorCheck::ServiceRunning => {
                    format!("  {} Service running", "✓".success())
                }
                DoctorCheck::ServiceStopped => {
                    format!(
                        "  {} Service stopped {} run `oneiros service start`",
                        "−".muted(),
                        "—".muted()
                    )
                }
                DoctorCheck::ServiceNotInstalled => {
                    format!(
                        "  {} Service not installed {} run `oneiros service install`",
                        "−".muted(),
                        "—".muted()
                    )
                }
            };
            lines.push(line);
        }

        lines.join("\n")
    }
}
