use clap::Subcommand;

use crate::*;

/// CLI subcommands for the tenant domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub(crate) enum TenantCommands {
    Create(CreateTenant),
    Get(GetTenant),
    List(ListTenants),
}

impl TenantCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, TenantError> {
        let client = Client::from_config(config)?;
        let tenant_client = TenantClient::new(&client);

        let response = match self {
            Self::Create(creation) => tenant_client.create(creation).await?,
            Self::Get(lookup) => tenant_client.get(lookup).await?,
            Self::List(listing) => tenant_client.list(listing).await?,
        };

        Ok(TenantView::new(response).render().map(Into::into))
    }
}
