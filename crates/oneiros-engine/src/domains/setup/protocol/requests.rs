use bon::Builder;
use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SetupRequest {
    /// Name for the system/host. Defaults to "oneiros user".
    #[arg(long)]
    #[builder(into)]
    pub(crate) name: Option<String>,
    /// Skip all confirmation prompts.
    #[arg(long, short)]
    #[builder(default)]
    pub(crate) yes: bool,
}
