use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum InitProject {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)]
            pub(crate) name: Option<BrainName>,
            #[arg(long, short)]
            #[serde(default)]
            #[builder(default)]
            pub(crate) yes: bool,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
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
    pub(crate) enum ImportProject {
        #[derive(clap::Args)]
        V1 => {
            pub(crate) file: PathBuf,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(
    kind = ProjectRequestType,
    display = "kebab-case",
    attrs(
        expect(
            clippy::enum_variant_names,
            reason = "We use these for `type` notation in serde"
        )
    )
)]
#[expect(
    clippy::enum_variant_names,
    reason = "We use these for `type` notation in serde"
)]
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
