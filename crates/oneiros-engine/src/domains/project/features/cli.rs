use std::path::PathBuf;

use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    Init {
        #[arg(long, short)]
        yes: bool,
    },
    Export {
        #[arg(long, short)]
        target: PathBuf,
    },
    Import {
        file: PathBuf,
    },
    Replay,
}

impl ProjectCommands {
    pub async fn execute(
        &self,
        ctx: &SystemContext,
        project: Option<&ProjectContext>,
        brain_name: BrainName,
    ) -> Result<Rendered<Responses>, ProjectError> {
        let response = match self {
            ProjectCommands::Init { .. } => ProjectService::init(ctx, brain_name).await?,
            ProjectCommands::Export { target } => {
                let project = project.ok_or(ProjectError::Missing)?;
                ProjectService::export(project, &target, &brain_name)?
            }
            ProjectCommands::Import { file } => {
                let project = project.ok_or(ProjectError::Missing)?;
                ProjectService::import(project, &file)?
            }
            ProjectCommands::Replay => {
                let project = project.ok_or(ProjectError::Missing)?;
                ProjectService::replay(project)?
            }
        };

        let prompt = match &response {
            ProjectResponse::BrainCreated(name) => format!("Brain '{name}' created."),
            ProjectResponse::BrainAlreadyExists(name) => format!("Brain '{name}' already exists."),
            ProjectResponse::WroteExport(path) => {
                format!("Export written to '{}'.", path.display())
            }
            ProjectResponse::Imported(result) => format!(
                "Imported {} events, replayed {}.",
                result.imported, result.replayed
            ),
            ProjectResponse::Replayed(result) => format!("Replayed {} events.", result.replayed),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
