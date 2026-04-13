use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SearchQuery {
    #[builder(into)]
    pub(crate) query: String,
    #[arg(long)]
    pub(crate) agent: Option<AgentName>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SearchRequestType, display = "kebab-case")]
pub(crate) enum SearchRequest {
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
