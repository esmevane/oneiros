use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ProjectResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ProjectResponse {
    Created(ProjectCreatedResponse),
    Found(ProjectFoundResponse),
    Listed(ProjectsResponse),
    ProjectAlreadyExists(ProjectAlreadyExistsResponse),
    WroteExport(WroteExportResponse),
    Imported(ImportedResponse),
    Replayed(ReplayedResponse),
    Shared(ProjectSharedResponse),
    Followed(ProjectFollowedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ProjectCreatedResponse {
        V1 => {
            #[serde(flatten)] pub(crate) project: Project,
            pub(crate) token: Token,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ProjectFoundResponse {
        V1 => { #[serde(flatten)] pub(crate) project: Project }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ProjectsResponse {
        V1 => {
            pub(crate) items: Vec<Project>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ProjectAlreadyExistsResponse {
        V1 => {
            #[builder(into)] pub(crate) project_name: ProjectName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum WroteExportResponse {
        V1 => {
            pub(crate) path: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ImportedResponse {
        V1 => {
            pub(crate) imported: i64,
            pub(crate) replayed: i64,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ReplayedResponse {
        V1 => {
            pub(crate) replayed: i64,
        }
    }
}

/// Response to `project share` — contains the issued ticket and URI.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct ProjectShareResult {
    pub(crate) ticket: Ticket,
    pub(crate) uri: String,
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ProjectSharedResponse {
        V1 => {
            #[serde(flatten)] pub(crate) result: ProjectShareResult,
        }
    }
}

// Response to `project follow` — the created repository peer.
versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ProjectFollowedResponse {
        V1 => {
            pub(crate) peer_name: PeerName,
            pub(crate) peer_id: PeerId,
            pub(crate) project: ProjectName,
        }
    }
}
