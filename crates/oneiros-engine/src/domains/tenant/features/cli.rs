use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

#[derive(Debug, Subcommand)]
pub enum TenantCommands {
    Create { name: String },
    Get { id: String },
    List,
}

impl TenantCommands {
    pub fn execute(&self, context: &SystemContext) -> Result<Rendered<Responses>, TenantError> {
        let response = match self {
            TenantCommands::Create { name } => {
                TenantService::create(context, TenantName::new(name))?
            }
            TenantCommands::Get { id } => TenantService::get(context, &id.parse::<TenantId>()?)?,
            TenantCommands::List => TenantService::list(context)?,
        };

        let prompt = match &response {
            TenantResponse::Created(tenant) => format!("Tenant '{}' created.", tenant.name),
            TenantResponse::Found(tenant) => format!("Tenant '{}' ({})", tenant.name, tenant.id),
            TenantResponse::Listed(tenants) => format!("{} tenant(s) found.", tenants.len()),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
