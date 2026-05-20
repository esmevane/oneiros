use schemars::JsonSchema;

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ParseLens {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: String,
        }
    }
}

impl ClientRequest for ParseLens {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/lens/parse", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ExplainLens {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: String,
        }
    }
}

impl ClientRequest for ExplainLens {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/lens/explain", self).await
    }
}
