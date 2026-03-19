use std::path::PathBuf;

use clap::Subcommand;

use crate::*;

pub struct ProjectCli;

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

impl ProjectCli {
    pub fn execute(
        ctx: &SystemContext,
        project: Option<&ProjectContext>,
        brain_name: &str,
        cmd: ProjectCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match cmd {
            ProjectCommands::Init { .. } => {
                let result = ProjectService::init(ctx, brain_name.to_string())?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
            ProjectCommands::Export { target } => {
                let project =
                    project.ok_or("project context required — call start_service first")?;
                let result = ProjectService::export(project, &target, brain_name)?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
            ProjectCommands::Import { file } => {
                let project =
                    project.ok_or("project context required — call start_service first")?;
                let result = ProjectService::import(project, &file)?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
            ProjectCommands::Replay => {
                let project =
                    project.ok_or("project context required — call start_service first")?;
                let result = ProjectService::replay(project)?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
        }
    }
}
