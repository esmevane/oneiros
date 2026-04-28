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
            ProjectResponse::Initialized(InitializedResponse::V1(details)) => {
                let prompt = Confirmation::new("Brain", details.brain_name.to_string(), "created")
                    .to_string();
                Rendered::new(
                    ProjectResponse::Initialized(
                        InitializedResponse::builder_v1()
                            .brain_name(details.brain_name)
                            .token(details.token)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::BrainAlreadyExists(BrainAlreadyExistsResponse::V1(details)) => {
                let prompt = format!(
                    "{}",
                    format!("Brain '{}' already exists.", details.brain_name).muted()
                );
                Rendered::new(
                    ProjectResponse::BrainAlreadyExists(
                        BrainAlreadyExistsResponse::builder_v1()
                            .brain_name(details.brain_name)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::WroteExport(WroteExportResponse::V1(details)) => {
                let prompt = format!(
                    "{} Export written to '{}'.",
                    "✓".success(),
                    details.path.display()
                );
                Rendered::new(
                    ProjectResponse::WroteExport(
                        WroteExportResponse::builder_v1()
                            .path(details.path)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::Imported(ImportedResponse::V1(details)) => {
                let prompt = format!(
                    "{} Imported {} events, replayed {}.",
                    "✓".success(),
                    details.imported,
                    details.replayed,
                );
                Rendered::new(
                    ProjectResponse::Imported(
                        ImportedResponse::builder_v1()
                            .imported(details.imported)
                            .replayed(details.replayed)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::Replayed(ReplayedResponse::V1(details)) => {
                let prompt = format!("{} Replayed {} events.", "✓".success(), details.replayed);
                Rendered::new(
                    ProjectResponse::Replayed(
                        ReplayedResponse::builder_v1()
                            .replayed(details.replayed)
                            .build()
                            .into(),
                    ),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
