use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExportProjectOutcomes {
    #[outcome(message("Exporting project to '{}'.", .0.to_string_lossy()))]
    UsingPath(PathBuf),
    #[outcome(message("Exported {0} lines."))]
    Lines(usize),
    #[outcome(message("Exported project to '{}'.", .0.to_string_lossy()))]
    WroteExport(PathBuf),
}

#[derive(Clone, Args)]
pub struct ExportProject {
    /// The path to export the project to.
    #[arg(long, short)]
    pub target: Option<PathBuf>,
}

impl ExportProject {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ExportProjectOutcomes>, ProjectCommandError> {
        let Some(project_name) = context.project_name() else {
            return Err(ProjectCommandError::NoProject);
        };

        let mut outcomes = Outcomes::new();
        let files = context.files();
        let root = context.project_root().map(|root| root.to_path_buf());
        let output = self.target.clone();

        let target_directory = match (root, output) {
            (Some(root), Some(output)) => {
                if output.is_relative() {
                    root.join(output)
                } else {
                    output
                }
            }
            (Some(root), _) => root,
            _ => Err(ProjectCommandError::NoProject)?,
        };

        outcomes.emit(ExportProjectOutcomes::UsingPath(target_directory.clone()));

        let events = context
            .client()
            .export_brain(&context.ticket_token()?)
            .await?;

        let mut buffer = String::new();
        let mut lines = 0;

        for event in events {
            buffer.push_str(&serde_json::to_string(&event)?);
            buffer.push('\n');
            lines += 1;
        }

        outcomes.emit(ExportProjectOutcomes::Lines(lines));

        let file_name = format!(
            "{}-{}-export.jsonl",
            project_name,
            Timestamp::now().as_date_string()
        );
        let file_path = target_directory.join(file_name);

        files.ensure_dir(&target_directory)?;
        files.write(&file_path, buffer)?;

        outcomes.emit(ExportProjectOutcomes::WroteExport(file_path));

        Ok(outcomes)
    }
}
