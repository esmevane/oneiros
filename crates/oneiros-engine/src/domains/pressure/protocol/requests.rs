use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, extract::Path};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{agent}",
        summary: "Get pressure",
        description: "Retrieve the current cognitive pressure level for a specific context.",
        content: include_str!("../features/skills/pressure.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetPressure::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .input::<AgentPathParam<AgentName>>()
                        .response::<200, Json<ReadingsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetPressure {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

#[expect(deprecated)]
impl GetPressure {
    pub(crate) async fn handler(
        context: ProjectLog,
        Path(agent): Path<AgentName>,
    ) -> Result<Json<PressureResponse>, PressureError> {
        Ok(Json(
            PressureService::get(
                &context,
                &GetPressure::builder_v1().agent(agent).build().into(),
            )
            .await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List pressure readings",
        description: "See all current cognitive pressure measurements across the project.",
        content: include_str!("../features/skills/pressure.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListPressures::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                        .response::<200, Json<AllReadingsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListPressures {
        V1 => {}
    }
}

#[expect(deprecated)]
impl ListPressures {
    pub(crate) async fn handler(
        context: ProjectLog,
    ) -> Result<Json<PressureResponse>, PressureError> {
        Ok(Json(PressureService::list(&context).await?))
    }
}

resource_requests! {
    GetPressure => |this, client| {
        let GetPressure::V1(lookup) = this;
        client.get(&format!("/pressures/{}", lookup.agent)).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PressureRequestType, display = "kebab-case")]
pub(crate) enum PressureRequest {
    GetPressure(GetPressure),
    ListPressures(ListPressures),
}

resource_root! {
    PressureRequest => {
        label: "pressure",
        purpose: "Monitor cognitive pressure levels",
        operations: [GetPressure, ListPressures],
        prefix: "pressures",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (PressureRequestType::GetPressure, "get-pressure"),
            (PressureRequestType::ListPressures, "list-pressures"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
