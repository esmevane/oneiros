use kinded::Kinded;
use noteworthy::annotation;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, extract::Query};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Search the project",
        description: "Query across agents, cognitions, memories, and other entities in the project.",
        content: include_str!("../features/skills/search.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                SearchQuery::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<SearchResponse>>()
                },
            )
        },
    })]
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

#[expect(deprecated)]
impl SearchQuery {
    pub(crate) async fn handler(
        context: ProjectLog,
        Query(params): Query<SearchQuery>,
    ) -> Result<Json<SearchResponse>, SearchError> {
        Ok(Json(SearchService::search(&context, &params).await?))
    }
}

resource_requests! {
    SearchQuery => |this, client| {
        let SearchQuery::V1(query) = this;
        let path = QueryPath::new("/search")
            .maybe_param("query", query.query.as_ref())
            .maybe_param("agent", query.agent.as_ref())
            .maybe_param("kind", query.kind.map(|kind| kind.as_str()))
            .maybe_param("texture", query.texture.as_ref())
            .maybe_param("level", query.level.as_ref())
            .maybe_param("sensation", query.sensation.as_ref())
            .param("limit", query.filters.limit)
            .param("offset", query.filters.offset)
            .finish();
        client.get(&path).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SearchRequestType, display = "kebab-case")]
pub(crate) enum SearchRequest {
    SearchQuery(SearchQuery),
}

resource_root! {
    SearchRequest => {
        label: "search",
        purpose: "Search across all entities in a project",
        operations: [SearchQuery],
    }
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
