use std::net::SocketAddr;

use bon::Builder;
use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct InitMcp {
    /// Bearer token for MCP authentication. Read from disk if not provided.
    #[arg(long)]
    #[builder(into)]
    pub(crate) token: Option<Token>,
    /// Service address. Uses config default if not provided.
    #[arg(long)]
    pub(crate) address: Option<SocketAddr>,
    /// Skip confirmation prompts.
    #[arg(long, short)]
    #[builder(default)]
    pub(crate) yes: bool,
}
