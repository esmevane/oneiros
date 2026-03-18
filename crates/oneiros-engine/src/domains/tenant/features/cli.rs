use clap::Subcommand;

use crate::*;
use crate::contexts::SystemContext;

pub struct TenantCli;

#[derive(Debug, Subcommand)]
pub enum TenantCommands {
    Create { name: String },
    Get { id: String },
    List,
}

impl TenantCli {
    pub fn execute(
        ctx: &SystemContext,
        cmd: TenantCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            TenantCommands::Create { name } => {
                serde_json::to_string_pretty(&TenantService::create(ctx, name)?)?
            }
            TenantCommands::Get { id } => {
                serde_json::to_string_pretty(&TenantService::get(ctx, &id)?)?
            }
            TenantCommands::List => {
                serde_json::to_string_pretty(&TenantService::list(ctx)?)?
            }
        };
        Ok(result)
    }
}
