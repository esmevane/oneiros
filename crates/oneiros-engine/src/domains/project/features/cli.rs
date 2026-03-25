use std::path::PathBuf;

use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    Init {
        #[arg(long, short)]
        yes: bool,
        #[arg(long)]
        name: Option<BrainName>,
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
    pub async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, ProjectError> {
        let response: ProjectResponse = match self {
            ProjectCommands::Init { name, .. } => {
                let brain_name = name.clone().unwrap_or(config.brain.clone());
                ProjectService::init(&config.system(), brain_name).await?
            }
            ProjectCommands::Export { target } => {
                let project = config.project();
                ProjectService::export(&project, target, project.brain_name())?
            }
            ProjectCommands::Import { file } => {
                let project = config.project();
                ProjectService::import(&project, file)?
            }
            ProjectCommands::Replay => {
                let project = config.project();
                ProjectService::replay(&project)?
            }
        };

        let prompt = match &response {
            ProjectResponse::Initialized(result) => {
                format!("Brain '{}' created.", result.brain_name)
            }
            ProjectResponse::BrainAlreadyExists(name) => format!("Brain '{name}' already exists."),
            ProjectResponse::WroteExport(path) => {
                format!("Export written to '{path}'.")
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
