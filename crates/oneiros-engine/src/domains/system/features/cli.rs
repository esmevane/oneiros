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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            SystemCommands::Init { name, .. } => {
                let name = name.unwrap_or_else(|| "onerios user".to_string());
                SystemService::init(ctx, name)?.into()
            }
        };
        Ok(result)
    }
}
