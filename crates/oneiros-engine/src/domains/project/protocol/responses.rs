use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ProjectResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ProjectResponse {
    Initialized(InitializedResponse),
    BrainAlreadyExists(BrainAlreadyExistsResponse),
    WroteExport(WroteExportResponse),
    Imported(ImportedResponse),
    Replayed(ReplayedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum InitializedResponse {
        V1 => {
            #[builder(into)] pub(crate) brain_name: BrainName,
            pub(crate) token: Token,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BrainAlreadyExistsResponse {
        V1 => {
            #[builder(into)] pub(crate) brain_name: BrainName,
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
