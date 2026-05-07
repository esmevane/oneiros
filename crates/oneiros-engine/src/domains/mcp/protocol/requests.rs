use std::net::SocketAddr;

use schemars::JsonSchema;

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum InitMcp {
        #[derive(clap::Args)]
        V1 => {
            /// Bearer token for MCP authentication. Read from disk if not provided.
            #[arg(long)]
            #[builder(into)]
            pub(crate) token: Option<Token>,
            /// Service address. Uses config default if not provided.
            #[arg(long)]
            pub(crate) address: Option<SocketAddr>,
            /// Skip confirmation prompts.
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) yes: bool,
        }
    }
}
