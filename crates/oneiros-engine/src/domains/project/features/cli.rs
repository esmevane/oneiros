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
            ProjectCommands::Init(init) => ProjectService::init(&config.system(), init).await?,
            ProjectCommands::Export(export) => ProjectService::export(&config.project(), export)?,
            ProjectCommands::Import(import) => ProjectService::import(&config.project(), import)?,
            ProjectCommands::Replay => ProjectService::replay(&config.project())?,
        };

        Ok(ProjectView::new(response).render().map(Into::into))
    }
}
