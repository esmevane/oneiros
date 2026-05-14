use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ProjectCommands {
    Create(CreateProject),
    Get(GetProject),
    List(ListProjects),
    Export(ExportProject),
    Import(ImportProject),
    Replay,
}

impl ProjectCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, ProjectError> {
        let client = Client::from_config(config)?;

        let response: ProjectResponse = match self {
            ProjectCommands::Create(creation) => {
                let mut creation = creation.clone();
                let CreateProject::V1(ref mut details) = creation;
                if details.name.is_none() {
                    details.name = Some(config.project.clone());
                }
                let bytes = creation.execute_request(&client).await?;
                serde_json::from_slice(&bytes)?
            }
            ProjectCommands::Get(lookup) => {
                let bytes = lookup.execute_request(&client).await?;
                serde_json::from_slice(&bytes)?
            }
            ProjectCommands::List(listing) => {
                let bytes = listing.execute_request(&client).await?;
                serde_json::from_slice(&bytes)?
            }
            ProjectCommands::Export(exporting) => {
                let scope = ComposeScope::new(config.clone())
                    .bookmark(config.project.clone(), config.bookmark.clone())?;
                ProjectService::export(&scope, exporting).await?
            }
            ProjectCommands::Import(importing) => ProjectService::import(config, importing).await?,
            ProjectCommands::Replay => ProjectService::replay(config).await?,
        };

        Ok(ProjectView::new(response).render().map(Into::into))
    }
}
