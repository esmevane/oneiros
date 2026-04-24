use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SearchQuery {
        #[derive(clap::Args)]
        V1 => {
            /// Full-text query. When absent, the search browses by filters alone,
            /// ordered by creation time.
            #[builder(into)] pub query: Option<String>,
            #[arg(long)] pub agent: Option<AgentName>,
            #[arg(long)] pub kind: Option<SearchKind>,
            #[arg(long)] pub texture: Option<TextureName>,
            #[arg(long)] pub level: Option<LevelName>,
            #[arg(long)] pub sensation: Option<SensationName>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
            /// Whether to compute facet aggregations alongside hits. Internal —
            /// flipped on by [`SearchService`] for explicit search; left off by
            /// list endpoints that don't render the palace map.
            #[arg(skip)]
            #[serde(skip)]
            #[builder(default)]
            pub with_facets: bool,
        }
    }
}

impl SearchQueryV1 {
    /// Return a clone of this query with facet aggregations enabled.
    pub fn with_facets(&self) -> Self {
        Self {
            with_facets: true,
            ..self.clone()
        }
    }
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
