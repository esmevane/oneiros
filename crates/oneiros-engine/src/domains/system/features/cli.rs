use clap::Subcommand;

use crate::*;

pub struct SystemCli;

#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    Init {
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long, short)]
        yes: bool,
    },
}

impl SystemCli {
    pub fn execute(
        ctx: &SystemContext,
        cmd: SystemCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match cmd {
            SystemCommands::Init { name, .. } => {
                let name = name.unwrap_or_else(|| "onerios user".to_string());
                let result = SystemService::init(ctx, name)?;
                Ok(serde_json::to_string_pretty(&result)?)
            }
        }
    }
}
