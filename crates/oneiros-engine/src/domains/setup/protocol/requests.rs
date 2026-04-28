use schemars::JsonSchema;

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SetupRequest {
        #[derive(clap::Args)]
        V1 => {
            /// Name for the system/host. Defaults to "oneiros user".
            #[arg(long)]
            #[builder(into)]
            pub name: Option<String>,
            /// Skip all confirmation prompts.
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub yes: bool,
        }
    }
}
