use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    Init {
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long, short)]
        yes: bool,
    },
}

impl SystemCommands {
    pub fn execute(&self, ctx: &SystemContext) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match self {
            SystemCommands::Init { name, .. } => {
                let name = name.clone().unwrap_or_else(|| "onerios user".to_string());
                SystemService::init(ctx, name)?.into()
            }
        };
        Ok(result)
    }
}
