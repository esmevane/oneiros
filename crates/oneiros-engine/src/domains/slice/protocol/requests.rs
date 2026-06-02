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
        V1 => {
            #[builder(into)]
            pub(crate) name: SliceName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum DiffSlice {
        V1 => {
            #[builder(into)]
            pub(crate) source: SliceName,
            #[builder(into)]
            pub(crate) target: SliceName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BookmarkSlice {
        V1 => {
            #[builder(into)]
            pub(crate) slice_name: SliceName,
            #[builder(into)]
            pub(crate) as_bookmark: BookmarkName,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SliceRequestType, display = "kebab-case")]
pub(crate) enum SliceRequest {
    CreateSlice(CreateSlice),
    DeleteSlice(DeleteSlice),
    DiffSlice(DiffSlice),
    BookmarkSlice(BookmarkSlice),
}
