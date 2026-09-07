use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{Json, extract::Path, http::StatusCode};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Create a slice",
        description: "Create a standing lens-filtered view of the event stream, materializing matching events retroactively.",
        content: include_str!("../features/skills/create.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateSlice::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                },
            )
        },
    })]
    pub(crate) enum CreateSlice {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) name: SliceName,
            #[builder(into)]
            #[arg(value_name = "LENS")]
            pub(crate) lens_expr: String,
        }
    }
}

#[expect(deprecated)]
impl CreateSlice {
    pub(crate) async fn handler(
        axum::extract::State(state): axum::extract::State<ServerState>,
        context: ProjectLog,
        mailbox: Mailbox,
        Json(body): Json<CreateSlice>,
    ) -> Result<(StatusCode, Json<SliceResponse>), SliceError> {
        let host_scope = ComposeScope::new(state.config().clone(), state.databases().clone())
            .host()
            .await?;
        let project_scope = context.scope().await?;
        Ok((
            StatusCode::CREATED,
            Json(
                SliceService::create(&host_scope, project_scope, &mailbox, state.canons(), &body)
                    .await?,
            ),
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List slices",
        description: "List all slices for the current project.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListSlices::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                },
            )
        },
    })]
    pub(crate) enum ListSlices {
        #[derive(clap::Args)]
        V1 => {}
    }
}

impl ListSlices {
    pub(crate) async fn handler(
        axum::extract::State(state): axum::extract::State<ServerState>,
    ) -> Result<Json<SliceResponse>, SliceError> {
        let host_scope = ComposeScope::new(state.config().clone(), state.databases().clone())
            .host()
            .await?;
        Ok(Json(SliceService::list(&host_scope).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Delete a slice",
        description: "Delete a slice by name.",
        content: include_str!("../features/skills/delete.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Delete,
        build: |docs| {
            ResourceMethod::Delete.router(
                DeleteSlice::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                },
            )
        },
    })]
    pub(crate) enum DeleteSlice {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) name: SliceName,
        }
    }
}

impl DeleteSlice {
    pub(crate) async fn handler(
        axum::extract::State(state): axum::extract::State<ServerState>,
        mailbox: Mailbox,
        Path(name): Path<SliceName>,
    ) -> Result<Json<SliceResponse>, SliceError> {
        let host_scope = ComposeScope::new(state.config().clone(), state.databases().clone())
            .host()
            .await?;
        Ok(Json(
            SliceService::delete(&host_scope, &mailbox, &name).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/diff",
        summary: "Diff two slices",
        description: "Compare two slices and return the event counts unique to each and shared between them.",
        content: include_str!("../features/skills/diff.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                DiffSlice::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.security_requirement("BearerToken")
                },
            )
        },
    })]
    pub(crate) enum DiffSlice {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)]
            pub(crate) source: SliceName,
            #[builder(into)]
            pub(crate) target: SliceName,
        }
    }
}

#[expect(deprecated)]
impl DiffSlice {
    pub(crate) async fn handler(
        axum::extract::State(state): axum::extract::State<ServerState>,
        context: ProjectLog,
        Json(body): Json<DiffSlice>,
    ) -> Result<Json<SliceResponse>, SliceError> {
        let host_scope = ComposeScope::new(state.config().clone(), state.databases().clone())
            .host()
            .await?;
        let project_scope = context.scope().await?;
        let DiffSlice::V1(req) = &body;
        Ok(Json(
            SliceService::diff(
                &host_scope,
                project_scope,
                state.canons(),
                &req.source,
                &req.target,
            )
            .await?,
        ))
    }
}

resource_requests! {
    CreateSlice => |this, client| { client.post("/slices", this).await },
    DeleteSlice => |this, client| {
        let DeleteSlice::V1(req) = this;
        client.delete(&format!("/slices/{}", req.name)).await
    },
    DiffSlice => |this, client| { client.post("/slices/diff", this).await },
}

resource_requests! {
    ListSlices => |client| { client.get("/slices").await },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SliceRequestType, display = "kebab-case")]
pub(crate) enum SliceRequest {
    CreateSlice(CreateSlice),
    ListSlices(ListSlices),
    DeleteSlice(DeleteSlice),
    DiffSlice(DiffSlice),
}

resource_root! {
    SliceRequest => {
        label: "slices",
        purpose: "Standing lens-filtered views over continuity",
        operations: [CreateSlice, ListSlices, DeleteSlice, DiffSlice],
    }
}
