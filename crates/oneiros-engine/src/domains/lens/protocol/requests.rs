#![allow(clippy::enum_variant_names)]

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, extract::State};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/parse",
        summary: "Parse a lens",
        description: "Parse a lens query expression and return its round-trip display form.",
        content: include_str!("../features/skills/parse.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                ParseLens::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<LensResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ParseLens {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: String,
        }
    }
}

impl ParseLens {
    pub(crate) async fn handler(
        Json(body): Json<ParseLens>,
    ) -> Result<Json<LensResponse>, LensError> {
        Ok(Json(LensService::parse(&body)?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/explain",
        summary: "Explain a lens",
        description: "Parse, validate, and compile a lens expression, returning the intermediate representation.",
        content: include_str!("../features/skills/explain.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                ExplainLens::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<LensResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ExplainLens {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: String,
        }
    }
}

impl ExplainLens {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Json(body): Json<ExplainLens>,
    ) -> Result<Json<LensResponse>, LensError> {
        Ok(Json(LensService::explain(&scope, &body).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/query",
        summary: "Query a lens",
        description: "Execute a lens query and return matching hits from the project.",
        content: include_str!("../features/skills/query.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                QueryLens::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<LensResponse>>()
                },
            )
        },
    })]
    pub(crate) enum QueryLens {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: String,
        }
    }
}

impl QueryLens {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        scope: Scope<AtBookmark>,
        Json(body): Json<QueryLens>,
    ) -> Result<Json<LensResponse>, LensError> {
        Ok(Json(
            LensService::query(&scope, state.canons(), &body).await?,
        ))
    }
}

resource_requests! {
    ParseLens => |this, client| {
        client.post("/lens/parse", this).await
    },
    ExplainLens => |this, client| {
        client.post("/lens/explain", this).await
    },
    QueryLens => |this, client| {
        client.post("/lens/query", this).await
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = LensRequestType, display = "kebab-case")]
pub(crate) enum LensRequest {
    ParseLens(ParseLens),
    ExplainLens(ExplainLens),
    QueryLens(QueryLens),
}

resource_root! {
    LensRequest => {
        label: "lens",
        purpose: "Parse, explain, and execute lens query expressions",
        operations: [ParseLens, ExplainLens, QueryLens],
    }
}
