use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, extract::Path};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/of/{ref}",
        summary: "Events that touched an entity",
        description: "Return the events that touched the entity at this ref, oldest first.",
        content: include_str!("../features/skills/of.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                TrailOf::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                },
            )
        },
    })]
    pub(crate) enum TrailOf {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            #[serde(rename = "ref")]
            #[arg(name = "ref")]
            pub(crate) r#ref: RefToken,
        }
    }
}

impl TrailOf {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(entity_ref): Path<RefToken>,
    ) -> Result<Json<TrailResponse>, TrailError> {
        Ok(Json(TrailService::of(&scope, &entity_ref).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/from/{event_id}",
        summary: "Entities emitted by an event",
        description: "Return the entity refs that this event emitted.",
        content: include_str!("../features/skills/from.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                TrailFrom::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                },
            )
        },
    })]
    pub(crate) enum TrailFrom {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) event: EventId,
        }
    }
}

impl TrailFrom {
    pub(crate) async fn handler(
        scope: Scope<AtBookmark>,
        Path(event_id): Path<EventId>,
    ) -> Result<Json<TrailResponse>, TrailError> {
        Ok(Json(TrailService::from(&scope, event_id).await?))
    }
}

resource_requests! {
    TrailOf => |this, client| {
        let TrailOf::V1(of) = this;
        client.get(&format!("/trail/of/{}", of.r#ref)).await
    },
    TrailFrom => |this, client| {
        let TrailFrom::V1(from) = this;
        client.get(&format!("/trail/from/{}", from.event)).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TrailRequestType, display = "kebab-case")]
pub(crate) enum TrailRequest {
    TrailOf(TrailOf),
    TrailFrom(TrailFrom),
}

resource_root! {
    TrailRequest => {
        label: "trail",
        purpose: "Walk the events ↔ entities bridge",
        operations: [TrailOf, TrailFrom],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        assert_eq!(&TrailRequestType::TrailOf.to_string(), "trail-of");
        assert_eq!(&TrailRequestType::TrailFrom.to_string(), "trail-from");
    }
}
