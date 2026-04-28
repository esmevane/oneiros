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
                ProjectService::init(&config.system(), initialization).await?
            }
            ProjectCommands::Export(exporting) => {
                ProjectService::export(&config.project(), exporting)?
            }
            ProjectCommands::Import(importing) => {
                ProjectService::import(&config.project(), importing)?
            }
            ProjectCommands::Replay => ProjectService::replay(&config.project())?,
        };

        Ok(ProjectView::new(response).render().map(Into::into))
    }
}
