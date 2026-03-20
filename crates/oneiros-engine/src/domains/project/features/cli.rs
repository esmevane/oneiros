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
    pub fn execute(
        &self,
        ctx: &SystemContext,
        project: Option<&ProjectContext>,
        brain_name: &str,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match self {
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
