use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

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

resource_requests! {
    CreateSlice => |this, client| { client.post("/slices", this).await },
    DeleteSlice => |this, client| {
        let DeleteSlice::V1(req) = this;
        client.delete(&format!("/slices/{}", req.name)).await
    },
    DiffSlice => |this, client| { client.post("/slices/diff", this).await },
}

resource_requests! {
    ListSlices => |client| { client.get("/slices").await },
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
