use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SearchQuery {
    pub query: String,
    #[arg(long)]
    pub agent: Option<AgentName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SearchRequest {
    Search(SearchQuery),
}
