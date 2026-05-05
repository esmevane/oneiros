use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    Init(InitProject),
    Export(ExportProject),
    Import(ImportProject),
    Replay,
}

impl ProjectCommands {
    pub async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, ProjectError> {
        let response: ProjectResponse = match self {
            ProjectCommands::Init(initialization) => {
                let client = config.system().client();
                ProjectClient::new(&client).init(initialization).await?
            }
            ProjectCommands::Export(exporting) => {
                let scope = ComposeScope::new(config.clone())
                    .bookmark(config.brain.clone(), config.bookmark.clone())?;
                ProjectService::export(&scope, exporting).await?
            }
            ProjectCommands::Import(importing) => ProjectService::import(config, importing).await?,
            ProjectCommands::Replay => ProjectService::replay(config).await?,
        };

        Ok(ProjectView::new(response).render().map(Into::into))
    }
}
