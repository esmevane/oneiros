use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

impl ClientRequest for CreateSlice {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/slices", self).await
    }
}

impl ClientRequest for ListSlices {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.get("/slices").await
    }
}

impl ClientRequest for DeleteSlice {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let DeleteSlice::V1(req) = self;
        client.delete(&format!("/slices/{}", req.name)).await
    }
}

impl ClientRequest for DiffSlice {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/slices/diff", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateSlice {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) name: SliceName,
            #[builder(into)]
            #[arg(value_name = "LENS")]
            pub(crate) lens_expr: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum DeleteSlice {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) name: SliceName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListSlices {
        #[derive(clap::Args)]
        V1 => {}
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum DiffSlice {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: SliceName,
            #[builder(into)]
            pub(crate) target: SliceName,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SliceRequestType, display = "kebab-case")]
pub(crate) enum SliceRequest {
    CreateSlice(CreateSlice),
    ListSlices(ListSlices),
    DeleteSlice(DeleteSlice),
    DiffSlice(DiffSlice),
}
