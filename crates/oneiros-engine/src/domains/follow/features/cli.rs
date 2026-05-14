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

        let bytes = match self {
            Self::Get(lookup) => lookup.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
        };

        let response: FollowResponse = serde_json::from_slice(&bytes)?;
        Ok(FollowView::new(response).render().map(Into::into))
    }
}
