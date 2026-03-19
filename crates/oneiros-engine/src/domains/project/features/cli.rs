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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            ProjectCommands::Init { .. } => {
                ProjectService::init(ctx, brain_name.to_string())?.into()
            }
            ProjectCommands::Export { target } => {
                let project =
                    project.ok_or("project context required — call start_service first")?;
                ProjectService::export(project, &target, brain_name)?.into()
            }
            ProjectCommands::Import { file } => {
                let project =
                    project.ok_or("project context required — call start_service first")?;
                ProjectService::import(project, &file)?.into()
            }
            ProjectCommands::Replay => {
                let project =
                    project.ok_or("project context required — call start_service first")?;
                ProjectService::replay(project)?.into()
            }
        };
        Ok(result)
    }
}
