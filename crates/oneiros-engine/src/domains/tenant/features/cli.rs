use clap::Subcommand;

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
        let client = context.client();
        let tenant_client = TenantClient::new(&client);

        let response = match self {
            TenantCommands::Create(create) => tenant_client.create(create).await?,
            TenantCommands::Get(get) => tenant_client.get(&get.id).await?,
            TenantCommands::List => tenant_client.list().await?,
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
