use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Create a project",
        description: "Provision a new project on this host: insert it into the host index, open its event log and default bookmark, and issue an access token.",
        content: include_str!("../features/skills/create.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateProject::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<201, Json<ProjectCreatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum CreateProject {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)]
            pub(crate) name: Option<ProjectName>,
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) yes: bool,
        }
    }
}

impl CreateProject {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<CreateProject>,
    ) -> Result<(StatusCode, Json<ProjectResponse>), ProjectError> {
        let response = ProjectService::create(&scope, &mailbox, &body).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{name}",
        summary: "Get a project",
        description: "Look up details for a specific project by name or ID.",
        content: include_str!("../features/skills/get.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetProject::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.input::<NamePathParam<ProjectName>>().response::<200, Json<ProjectFoundResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetProject {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ProjectName>,
        }
    }
}

impl GetProject {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<ProjectName>>,
    ) -> Result<Json<ProjectResponse>, ProjectError> {
        Ok(Json(
            ProjectService::get(&scope, &GetProject::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List projects",
        description: "List all projects provisioned on this host.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListProjects::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<ProjectsResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ListProjects {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ListProjects {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Query(params): Query<ListProjects>,
    ) -> Result<Json<ProjectResponse>, ProjectError> {
        Ok(Json(ProjectService::list(&scope, &params).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/export",
        summary: "Export a project",
        description: "Export the current project to a file.",
        content: include_str!("../features/skills/export.md"),
        status: 200,
    })]
    pub(crate) enum ExportProject {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long, short)]
            pub(crate) target: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/import",
        summary: "Import a project",
        description: "Import a project from a file.",
        content: include_str!("../features/skills/import.md"),
        status: 200,
    })]
    pub(crate) enum ImportProject {
        #[derive(clap::Args)]
        V1 => {
            pub(crate) file: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/replay",
        summary: "Replay a project",
        description: "Replay events from a project file.",
        content: include_str!("../features/skills/replay.md"),
        status: 200,
    })]
    pub(crate) enum ReplayProject {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long, short)]
            pub(crate) file: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/share",
        summary: "Share a project",
        description: "Issue a project-scoped ticket and print the URI for peer access.",
        content: include_str!("../features/skills/share.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                ShareProject::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<ProjectSharedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ShareProject {
        #[derive(clap::Args)]
        V1 => {
            #[arg(default_value = "")]
            #[builder(into)]
            pub(crate) project: ProjectName,
        }
    }
}

impl ShareProject {
    pub(crate) async fn handler(
        State(state): State<ServerState>,
        Json(body): Json<ShareProject>,
    ) -> Result<Json<ProjectResponse>, ProjectError> {
        Ok(Json(ProjectService::share(&state, &body).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/follow",
        summary: "Follow a project",
        description: "Create a repository peer by following a project share URI.",
        content: include_str!("../features/skills/follow.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                FollowProject::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<ProjectFollowedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum FollowProject {
        #[derive(clap::Args)]
        V1 => {
            pub(crate) uri: String,
            #[arg(long)]
            #[builder(into)]
            pub(crate) name: PeerName,
        }
    }
}

impl FollowProject {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<FollowProject>,
    ) -> Result<Json<ProjectResponse>, ProjectError> {
        Ok(Json(ProjectService::follow(&scope, &mailbox, &body).await?))
    }
}

/// GET /summary — project overview dashboard. Not an annotated resource.
pub(crate) async fn summary(
    scope: Scope<AtBookmark>,
) -> Result<Json<ProjectSummary>, ProjectError> {
    let db = scope.bookmark_db().await?;

    let agents = AgentStore::new(&db).list().unwrap_or_default();
    let agent_count = agents.len();

    let cognitions = CognitionStore::new(&db)
        .list(None, None)
        .unwrap_or_default();
    let cognition_count = cognitions.len();

    let recent_cognitions = {
        let mut recent = cognitions;
        recent.sort_by_key(|b| std::cmp::Reverse(b.created_at));
        recent.truncate(30);
        recent
    };

    let memory_count = MemoryStore::new(&db).list(None).unwrap_or_default().len();
    let experience_count = ExperienceStore::new(&db)
        .list(None)
        .unwrap_or_default()
        .len();
    let connection_count = ConnectionStore::new(&db)
        .list(None)
        .unwrap_or_default()
        .len();

    let event_count = db
        .query_row("SELECT COUNT(*) FROM events", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap_or(0) as usize;

    let summary = ProjectSummary {
        agents,
        agent_count,
        cognition_count,
        memory_count,
        experience_count,
        connection_count,
        event_count,
        recent_cognitions,
    };

    Ok(Json(summary))
}

resource_requests! {
    CreateProject => |this, client| {
        client.post("/projects", this).await
    },
    GetProject => |this, client| {
        let GetProject::V1(lookup) = this;
        client.get(&format!("/projects/{}", lookup.key)).await
    },
    ListProjects => |this, client| {
        let ListProjects::V1(listing) = this;
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset,);
        client.get(&format!("/projects?{query}")).await
    },
    ShareProject => |this, client| {
        client.post("/projects/share", this).await
    },
    FollowProject => |this, client| {
        client.post("/projects/follow", this).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ProjectRequestType, display = "kebab-case")]
pub(crate) enum ProjectRequest {
    CreateProject(CreateProject),
    GetProject(GetProject),
    ListProjects(ListProjects),
    ExportProject(ExportProject),
    ImportProject(ImportProject),
    ReplayProject(ReplayProject),
    ShareProject(ShareProject),
    FollowProject(FollowProject),
}

resource_root! {
    ProjectRequest => {
        label: "projects",
        purpose: "Manage projects on this host",
        operations: [CreateProject, GetProject, ListProjects, ShareProject, FollowProject],
        skills: [ExportProject, ImportProject, ReplayProject],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ProjectRequestType::CreateProject, "create-project"),
            (ProjectRequestType::GetProject, "get-project"),
            (ProjectRequestType::ListProjects, "list-projects"),
            (ProjectRequestType::ExportProject, "export-project"),
            (ProjectRequestType::ImportProject, "import-project"),
            (ProjectRequestType::ReplayProject, "replay-project"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
