//! Doctor view — presentation for health check results.

use crate::*;

pub(crate) struct DoctorView {
    response: DoctorResponse,
}

impl DoctorView {
    pub(crate) fn new(response: DoctorResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<DoctorResponse> {
        match self.response {
            DoctorResponse::CheckupStatus(CheckupStatusResponse::V1(details)) => {
                let prompt = Self::checklist(&details.checks);
                Rendered::new(
                    DoctorResponse::CheckupStatus(
                        CheckupStatusResponse::builder_v1()
                            .checks(details.checks)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
        }
    }

    fn checklist(checks: &[DoctorCheck]) -> String {
        let mut lines = vec![format!("{}", "Oneiros health check:".heading())];

        for check in checks {
            let line = match check {
                DoctorCheck::Initialized => {
                    format!("  {} Host initialized", "✓".success())
                }
                DoctorCheck::NotInitialized => {
                    format!(
                        "  {} Host not initialized {} run `oneiros setup`",
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
                DoctorCheck::ProjectExists(name) => {
                    format!("  {} Project '{}' exists", "✓".success(), name)
                }
                DoctorCheck::ProjectMissing(name) => {
                    format!(
                        "  {} Project '{}' not found {} run `oneiros project create`",
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
                        "  {} Host keypair missing {} run `oneiros host init`",
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
