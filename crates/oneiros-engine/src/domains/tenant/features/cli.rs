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
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, TenantError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Create(creation) => creation.execute_request(&client).await?,
            Self::Get(lookup) => lookup.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
        };

        let response: TenantResponse = serde_json::from_slice(&bytes)?;
        Ok(TenantView::new(response).render().map(Into::into))
    }
}
