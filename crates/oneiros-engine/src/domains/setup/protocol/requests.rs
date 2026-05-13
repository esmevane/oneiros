use schemars::JsonSchema;

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetupRequest {
        #[derive(clap::Args)]
        V1 => {
            /// Name for the host. Defaults to "oneiros user".
            #[arg(long)]
            #[builder(into)]
            pub(crate) name: Option<String>,
            /// Skip all confirmation prompts.
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) yes: bool,
        }
    }
}
