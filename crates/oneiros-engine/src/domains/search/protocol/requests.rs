use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SearchQuery {
    /// Full-text query. When absent, the search browses by filters alone,
    /// ordered by creation time.
    #[builder(into)]
    pub query: Option<String>,

    #[arg(long)]
    pub agent: Option<AgentName>,

    #[arg(long)]
    pub kind: Option<SearchKind>,

    #[arg(long)]
    pub texture: Option<TextureName>,

    #[arg(long)]
    pub level: Option<LevelName>,

    #[arg(long)]
    pub sensation: Option<SensationName>,

    #[command(flatten)]
    #[serde(flatten, default)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SearchRequestType, display = "kebab-case")]
pub enum SearchRequest {
    SearchQuery(SearchQuery),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [(SearchRequestType::SearchQuery, "search-query")];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
