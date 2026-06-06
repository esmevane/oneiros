use clap::Subcommand;
use schemars::JsonSchema;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum RemoteCommands {
    Add(AddRemote),
    List(ListRemotes),
    Remove(RemoveRemote),
    Bookmarks(RemoteBookmarks),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoteBookmarks {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: RemoteName,
        }
    }
}

impl RemoteCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, RemoteError> {
        let client = Client::from_config(config)?;
        let bytes = match self {
            Self::Add(add) => add.execute_request(&client).await?,
            Self::List(list) => list.execute_request(&client).await?,
            Self::Remove(remove) => remove.execute_request(&client).await?,
            Self::Bookmarks(bm) => {
                let RemoteBookmarks::V1(bm) = bm;
                client
                    .get(&format!("/remotes/{}/bookmarks", bm.name))
                    .await?
            }
        };
        let response: RemoteResponse = serde_json::from_slice(&bytes)?;
        Ok(RemoteView::new(response).render().map(Into::into))
    }
}
