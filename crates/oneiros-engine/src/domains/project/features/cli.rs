use clap::Subcommand;

use crate::*;

pub struct ProjectCli;

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    Init {
        #[arg(long, short)]
        yes: bool,
    },
}

impl ProjectCli {
    pub fn execute(
        ctx: &SystemContext,
        brain_name: &str,
        cmd: ProjectCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match cmd {
            ProjectCommands::Init { .. } => {
                let result = ProjectService::init(ctx, brain_name.to_string())?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
        }
    }
}
