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
            /// Install and start the host service if it isn't running.
            #[arg(long)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) install_host: bool,
            /// Write .mcp.json for Claude Code integration.
            #[arg(long)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) init_mcp: bool,
            /// Accept all optional steps without prompting.
            #[arg(long, short = 'y')]
            #[serde(default)]
            #[builder(default)]
            pub(crate) accept_all: bool,
        }
    }
}
