use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
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

impl ClientRequest for CreateProject {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/projects", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetProject {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ProjectName>,
        }
    }
}

impl ClientRequest for GetProject {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetProject::V1(lookup) = self;
        client.get(&format!("/projects/{}", lookup.key)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
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

impl ClientRequest for ListProjects {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListProjects::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/projects?{query}")).await
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
#[kinded(kind = ProjectRequestType, display = "kebab-case")]
pub(crate) enum ProjectRequest {
    CreateProject(CreateProject),
    GetProject(GetProject),
    ListProjects(ListProjects),
    ExportProject(ExportProject),
    ImportProject(ImportProject),
    ReplayProject,
    ShareProject(ShareProject),
    FollowProject(FollowProject),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ShareProject {
        #[derive(clap::Args)]
        V1 => {
            #[arg(default_value = "")]
            #[builder(into)]
            pub(crate) project: ProjectName,
        }
    }
}

impl ClientRequest for ShareProject {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/projects/share", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
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

impl ClientRequest for FollowProject {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/projects/follow", self).await
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
