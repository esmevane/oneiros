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

        Ok(TenantView::new(response).render().map(Into::into))
    }
}
