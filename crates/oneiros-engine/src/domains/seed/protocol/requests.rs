use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, http::StatusCode};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/core",
        summary: "Seed core vocabulary",
        description: "Plant the foundational textures, sensations, urges, and personas into the project.",
        content: include_str!("../features/skills/core.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                SeedCore::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<SeedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SeedCore {
        V1 => {}
    }
}

impl SeedCore {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
    ) -> Result<(StatusCode, Json<SeedResponse>), SeedError> {
        let response = SeedService::core(&scope, &mailbox).await?;
        Ok((StatusCode::OK, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/agents",
        summary: "Seed agents",
        description: "Plant a default set of agents into the project to bootstrap cognition.",
        content: include_str!("../features/skills/agents.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                SeedAgents::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken").response::<200, Json<SeedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum SeedAgents {
        V1 => {}
    }
}

impl SeedAgents {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        mailbox: Mailbox,
    ) -> Result<(StatusCode, Json<SeedResponse>), SeedError> {
        let response = SeedService::agents(&scope, &mailbox).await?;
        Ok((StatusCode::OK, Json(response)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SeedRequestType, display = "kebab-case")]
pub(crate) enum SeedRequest {
    SeedCore(SeedCore),
    SeedAgents(SeedAgents),
}

resource_root! {
    SeedRequest => {
        label: "seed",
        purpose: "Plant initial vocabulary and agents",
        operations: [SeedCore, SeedAgents],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (SeedRequestType::SeedCore, "seed-core"),
            (SeedRequestType::SeedAgents, "seed-agents"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
