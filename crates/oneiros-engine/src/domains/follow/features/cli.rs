use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum FollowCommands {
    Get(GetFollow),
    List(ListFollows),
}

impl FollowCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, FollowError> {
        let client = Client::from_config(config)?;
        let follow_client = FollowClient::new(&client);

        let response = match self {
            Self::Get(lookup) => follow_client.get(lookup).await?,
            Self::List(listing) => follow_client.list(listing).await?,
        };

        Ok(FollowView::new(response).render().map(Into::into))
    }
}
