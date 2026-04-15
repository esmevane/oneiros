use std::net::SocketAddr;

use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct InitMcp {
    /// Bearer token for MCP authentication. Read from disk if not provided.
    #[arg(long)]
    #[builder(into)]
    pub token: Option<Token>,
    /// Service address. Uses config default if not provided.
    #[arg(long)]
    pub address: Option<SocketAddr>,
    /// Skip confirmation prompts.
    #[arg(long, short)]
    #[builder(default)]
    pub yes: bool,
}

/// Request to load a named toolset into the current MCP session.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActivateToolsetRequest {
    /// The toolset to activate: lifecycle, continuity, vocabulary, administer, or manage
    pub name: String,
}

/// Request to unload the active toolset from the current MCP session.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeactivateToolsetRequest {}

/// Toolset management request variants.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = ToolsetRequestType, display = "kebab-case")]
pub enum ToolsetRequest {
    ActivateToolset(ActivateToolsetRequest),
    DeactivateToolset(DeactivateToolsetRequest),
}
