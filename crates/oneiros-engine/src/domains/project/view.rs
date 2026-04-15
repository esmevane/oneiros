//! Project view — presentation authority for the project domain.
//!
//! Maps project responses into formatted strings using shared view primitives.
//! The domain knows its own shape; the rendering layer decides how to display it.

use crate::*;

pub struct ProjectView {
    response: ProjectResponse,
}

impl ProjectView {
    pub fn new(response: ProjectResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<ProjectResponse> {
        match self.response {
            ProjectResponse::Initialized(result) => {
                let prompt = Confirmation::new("Brain", result.brain_name.to_string(), "created")
                    .to_string();
                Rendered::new(ProjectResponse::Initialized(result), prompt, String::new())
            }
            ProjectResponse::BrainAlreadyExists(name) => {
                let prompt = format!("{}", format!("Brain '{name}' already exists.").muted());
                Rendered::new(
                    ProjectResponse::BrainAlreadyExists(name),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::WroteExport(path) => {
                let prompt = format!("{} Export written to '{path}'.", "✓".success());
                Rendered::new(ProjectResponse::WroteExport(path), prompt, String::new())
            }
            ProjectResponse::Imported(result) => {
                let prompt = format!(
                    "{} Imported {} events, replayed {}.",
                    "✓".success(),
                    result.imported,
                    result.replayed,
                );
                Rendered::new(ProjectResponse::Imported(result), prompt, String::new())
            }
            ProjectResponse::Replayed(result) => {
                let prompt = format!("{} Replayed {} events.", "✓".success(), result.replayed);
                Rendered::new(ProjectResponse::Replayed(result), prompt, String::new())
            }
        }
    }
}
