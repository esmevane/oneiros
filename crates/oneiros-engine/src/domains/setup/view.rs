//! Setup view — presentation for setup step results.

use crate::*;

pub struct SetupView {
    response: SetupResponse,
}

impl SetupView {
    pub fn new(response: SetupResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<SetupResponse> {
        match self.response {
            SetupResponse::SetupComplete(SetupCompleteResponse::V1(details)) => {
                let prompt = Self::steps(&details.steps);
                Rendered::new(
                    SetupResponse::SetupComplete(
                        SetupCompleteResponse::builder_v1()
                            .steps(details.steps)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
        }
    }

    fn steps(steps: &[SetupStep]) -> String {
        let mut lines = vec![format!("{}", "Setup complete.".heading())];

        for step in steps {
            let line = match step {
                SetupStep::SystemInitialized => {
                    format!("  {} System initialized", "✓".success())
                }
                SetupStep::SystemAlreadyInitialized => {
                    format!("  {} System already initialized", "=".muted())
                }
                SetupStep::ProjectInitialized(name) => {
                    format!("  {} Brain '{}' created", "✓".success(), name)
                }
                SetupStep::ProjectAlreadyExists(name) => {
                    format!("  {} Brain '{}' already exists", "=".muted(), name)
                }
                SetupStep::VocabularySeeded => {
                    format!("  {} Vocabulary seeded", "✓".success())
                }
                SetupStep::AgentsSeeded => {
                    format!("  {} Agents seeded", "✓".success())
                }
                SetupStep::McpConfigured => {
                    format!("  {} MCP config written", "✓".success())
                }
                SetupStep::McpSkipped => {
                    format!("  {} MCP config skipped", "−".muted())
                }
                SetupStep::ServiceInstalled => {
                    format!("  {} Service installed", "✓".success())
                }
                SetupStep::ServiceStarted => {
                    format!("  {} Service started", "✓".success())
                }
                SetupStep::ServiceSkipped => {
                    format!("  {} Service skipped", "−".muted())
                }
                SetupStep::StepFailed { step, reason } => {
                    format!("  {} {} failed: {}", "!".error(), step, reason)
                }
            };
            lines.push(line);
        }

        lines.join("\n")
    }
}
