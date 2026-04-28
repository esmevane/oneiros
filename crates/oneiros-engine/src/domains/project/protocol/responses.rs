use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ProjectResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ProjectResponse {
    Initialized(InitializedResponse),
    BrainAlreadyExists(BrainAlreadyExistsResponse),
    WroteExport(WroteExportResponse),
    Imported(ImportedResponse),
    Replayed(ReplayedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum InitializedResponse {
        V1 => {
            #[builder(into)] pub brain_name: BrainName,
            pub token: Token,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BrainAlreadyExistsResponse {
        V1 => {
            #[builder(into)] pub brain_name: BrainName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum WroteExportResponse {
        V1 => {
            pub path: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ImportedResponse {
        V1 => {
            pub imported: i64,
            pub replayed: i64,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ReplayedResponse {
        V1 => {
            pub replayed: i64,
        }
    }
}
