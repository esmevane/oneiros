use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SearchQuery {
        #[derive(clap::Args)]
        V1 => {
            /// Full-text query. When absent, the search browses by filters alone,
            /// ordered by creation time.
            #[builder(into)] pub(crate) query: Option<String>,
            #[arg(long)] pub(crate) agent: Option<AgentName>,
            #[arg(long)] pub(crate) kind: Option<SearchKind>,
            #[arg(long)] pub(crate) texture: Option<TextureName>,
            #[arg(long)] pub(crate) level: Option<LevelName>,
            #[arg(long)] pub(crate) sensation: Option<SensationName>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
            /// Whether to compute facet aggregations alongside hits. Internal —
            /// flipped on by [`SearchService`] for explicit search; left off by
            /// list endpoints that don't render the palace map.
            #[arg(skip)]
            #[serde(skip)]
            #[builder(default)]
            pub(crate) with_facets: bool,
        }
    }
}

impl SearchQueryV1 {
    /// Return a clone of this query with facet aggregations enabled.
    pub(crate) fn with_facets(&self) -> Self {
        Self {
            with_facets: true,
            ..self.clone()
        }
    }
}

resource_requests! {
    SearchQuery => |this, client| {
        let SearchQuery::V1(query) = this;
        let mut parts: Vec<String> = Vec::new();
        if let Some(q) = &query.query {
            parts.push(format!("query={q}"));
        }
        if let Some(a) = &query.agent {
            parts.push(format!("agent={a}"));
        }
        if let Some(k) = query.kind {
            parts.push(format!("kind={}", k.as_str()));
        }
        if let Some(t) = &query.texture {
            parts.push(format!("texture={t}"));
        }
        if let Some(l) = &query.level {
            parts.push(format!("level={l}"));
        }
        if let Some(s) = &query.sensation {
            parts.push(format!("sensation={s}"));
        }
        parts.push(format!("limit={}", query.filters.limit));
        parts.push(format!("offset={}", query.filters.offset));

        let path = format!("/search?{}", parts.join("&"));
        client.get(&path).await
    },
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
