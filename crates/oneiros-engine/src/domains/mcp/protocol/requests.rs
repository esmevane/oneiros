use std::net::SocketAddr;

use schemars::JsonSchema;

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum InitMcp {
        #[derive(clap::Args)]
        V1 => {
            /// Bearer token for MCP authentication. Read from disk if not provided.
            #[arg(long)]
            #[builder(into)]
            pub token: Option<Token>,
            /// Service address. Uses config default if not provided.
            #[arg(long)]
            pub address: Option<SocketAddr>,
            /// Skip confirmation prompts.
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub yes: bool,
        }
    }
}
