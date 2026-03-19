use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            TenantCommands::Create { name } => TenantService::create(ctx, name)?.into(),
            TenantCommands::Get { id } => TenantService::get(ctx, &id)?.into(),
            TenantCommands::List => TenantService::list(ctx)?.into(),
        };
        Ok(result)
    }
}
