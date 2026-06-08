use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AddRemote {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: RemoteName,
            /// The project-scoped ticket URI (oneiros://host/link:...).
            #[arg(long)]
            #[builder(into)] pub(crate) ticket: String,
        }
    }
}

impl ClientRequest for AddRemote {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/remotes", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetRemote {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<RemoteId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListRemotes {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListRemotes {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListRemotes::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset
        );
        client.get(&format!("/remotes?{query}")).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveRemote {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: RemoteName,
        }
    }
}

impl ClientRequest for RemoveRemote {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemoveRemote::V1(remove) = self;
        client.delete(&format!("/remotes/{}", remove.name)).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = RemoteRequestType, display = "kebab-case")]
pub(crate) enum RemoteRequest {
    AddRemote(AddRemote),
    GetRemote(GetRemote),
    ListRemotes(ListRemotes),
    RemoveRemote(RemoveRemote),
    ShareRemote(ShareRemote),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ShareRemote {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) project: ProjectName,
        }
    }
}

impl ClientRequest for ShareRemote {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/remotes/share", self).await
    }
}
