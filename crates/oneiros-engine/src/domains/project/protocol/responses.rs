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
