//! Project view — presentation authority for the project domain.

use crate::*;

pub(crate) struct ProjectView {
    response: ProjectResponse,
}

impl ProjectView {
    pub(crate) fn new(response: ProjectResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<ProjectResponse> {
        match self.response {
            ProjectResponse::Created(ProjectCreatedResponse::V1(created)) => {
                let prompt =
                    Confirmation::new("Project", created.project.name.to_string(), "created")
                        .to_string();
                Rendered::new(
                    ProjectResponse::Created(ProjectCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::Found(ProjectFoundResponse::V1(found)) => {
                let prompt = Detail::new(found.project.name.to_string()).to_string();
                Rendered::new(
                    ProjectResponse::Found(ProjectFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::Listed(ProjectsResponse::V1(listed)) => {
                let mut table = Table::new(vec![Column::new("Name")]);
                for project in &listed.items {
                    table.push_row(vec![project.name.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    ProjectResponse::Listed(ProjectsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::ProjectAlreadyExists(ProjectAlreadyExistsResponse::V1(details)) => {
                let prompt = format!("Project '{}' already exists.", details.project_name)
                    .muted()
                    .to_string();
                Rendered::new(
                    ProjectResponse::ProjectAlreadyExists(
                        ProjectAlreadyExistsResponse::builder_v1()
                            .project_name(details.project_name)
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
            ProjectResponse::Shared(ProjectSharedResponse::V1(shared)) => {
                let prompt = shared.result.uri.clone();
                Rendered::new(
                    ProjectResponse::Shared(ProjectSharedResponse::V1(shared)),
                    prompt,
                    String::new(),
                )
            }
            ProjectResponse::Followed(ProjectFollowedResponse::V1(followed)) => {
                let prompt = Confirmation::new(
                    "Project",
                    followed.project.to_string(),
                    format!("followed as peer '{}'", followed.peer_name),
                )
                .to_string();
                Rendered::new(
                    ProjectResponse::Followed(ProjectFollowedResponse::V1(followed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
