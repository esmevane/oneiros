use std::path::PathBuf;

use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct InitProject {
    #[arg(long)]
    #[builder(into)]
    pub(crate) name: Option<BrainName>,
    #[arg(long, short)]
    #[builder(default)]
    pub(crate) yes: bool,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ExportProject {
    #[arg(long, short)]
    pub(crate) target: PathBuf,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ImportProject {
    pub(crate) file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ProjectRequestType, display = "kebab-case")]
pub(crate) enum ProjectRequest {
    InitProject(InitProject),
    ExportProject(ExportProject),
    ImportProject(ImportProject),
    ReplayProject,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ProjectRequestType::InitProject, "init-project"),
            (ProjectRequestType::ExportProject, "export-project"),
            (ProjectRequestType::ImportProject, "import-project"),
            (ProjectRequestType::ReplayProject, "replay-project"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
