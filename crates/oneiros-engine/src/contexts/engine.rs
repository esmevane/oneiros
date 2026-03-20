use crate::*;

/// The combined context for CLI execution.
///
/// Holds both system and project contexts. System context is always
/// available. Project context is available after `project init` and
/// `start_service`.
pub struct EngineContext {
    pub system: SystemContext,
    pub project: Option<ProjectContext>,
    pub brain_name: String,
}

impl EngineContext {
    pub fn project(&self) -> Result<&ProjectContext, Box<dyn std::error::Error>> {
        self.project
            .as_ref()
            .ok_or_else(|| "project context required — call start_service first".into())
    }
}
