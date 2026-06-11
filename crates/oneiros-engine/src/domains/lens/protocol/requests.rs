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

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum QueryLens {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: String,
        }
    }
}

resource_requests! {
    ParseLens => |this, client| {
        client.post("/lens/parse", this).await
    },
    ExplainLens => |this, client| {
        client.post("/lens/explain", this).await
    },
    QueryLens => |this, client| {
        client.post("/lens/query", this).await
    },
}
