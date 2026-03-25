use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

#[derive(Debug, Subcommand)]
pub enum TenantCommands {
    Create(CreateTenant),
    Get(GetTenant),
    List,
}

impl TenantCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, TenantError> {
        let response = match self {
            TenantCommands::Create(create) => {
                TenantService::create(context, create.name.clone()).await?
            }
            TenantCommands::Get(get) => TenantService::get(context, &get.id).await?,
            TenantCommands::List => TenantService::list(context).await?,
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
