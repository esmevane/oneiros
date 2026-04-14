use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum TenantCommands {
    Create(CreateTenant),
    Get(GetTenant),
    List(ListTenants),
}

impl TenantCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, TenantError> {
        
        let tenant_client = TenantClient::new(client);

        let response = match self {
            TenantCommands::Create(create) => tenant_client.create(create).await?,
            TenantCommands::Get(get) => tenant_client.get(&get.id).await?,
            TenantCommands::List(list) => tenant_client.list(list).await?,
        };

        let prompt = match &response {
            TenantResponse::Created(wrapped) => {
                TenantView::confirmed("created", &wrapped.data.name).to_string()
            }
            TenantResponse::Found(wrapped) => TenantView::detail(&wrapped.data).to_string(),
            TenantResponse::Listed(listed) => {
                let table = TenantView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
