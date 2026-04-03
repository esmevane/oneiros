use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum TenantCommands {
    Create(CreateTenant),
    Get(GetTenant),
    List(ListTenants),
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
            TenantCommands::List(list) => tenant_client.list(list).await?,
        };

        let prompt = match &response {
            TenantResponse::Created(tenant) => format!("Tenant '{}' created.", tenant.name),
            TenantResponse::Found(tenant) => format!("Tenant '{}' ({})", tenant.name, tenant.id),
            TenantResponse::Listed(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for tenant in &listed.items {
                    out.push_str(&format!("  {}\n\n", tenant.name));
                }
                out
            }
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
