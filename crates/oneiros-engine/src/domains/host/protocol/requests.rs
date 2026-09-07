use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, extract::State, http::StatusCode};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Initialize host",
        description: "Create the host data directory, generate the host keypair, and seed the default tenant and actor. Refuses once a tenant exists.",
        content: include_str!("../features/skills/init.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                InitHost::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<201, Json<HostResponse>>()
                },
            )
        },
    })]
    pub(crate) enum InitHost {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long, short)]
            #[builder(into)]
            pub(crate) name: Option<String>,
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) yes: bool,
        }
    }
}

impl InitHost {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        Json(body): Json<InitHost>,
    ) -> Result<(StatusCode, Json<HostResponse>), HostError> {
        let response =
            HostService::init(state.config(), state.databases(), state.mailbox(), &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

resource_requests! {
    InitHost => |this, client| {
        client.post("/host", this).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = HostRequestType, display = "kebab-case")]
pub(crate) enum HostRequest {
    InitHost(InitHost),
}

resource_root! {
    HostRequest => {
        label: "host",
        purpose: "Host-level initialization and bootstrap",
        operations: [InitHost],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [(HostRequestType::InitHost, "init-host")];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
