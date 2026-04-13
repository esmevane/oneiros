use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ProjectCommands {
    Init(InitProject),
    Export(ExportProject),
    Import(ImportProject),
    Replay,
}

impl ProjectCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, ProjectError> {
        let response: ProjectResponse = match self {
            ProjectCommands::Init(init) => ProjectService::init(&config.system(), init).await?,
            ProjectCommands::Export(export) => ProjectService::export(&config.project(), export)?,
            ProjectCommands::Import(import) => ProjectService::import(&config.project(), import)?,
            ProjectCommands::Replay => ProjectService::replay(&config.project())?,
        };

        let prompt = match &response {
            ProjectResponse::Initialized(result) => ProjectView::initialized(result),
            ProjectResponse::BrainAlreadyExists(name) => ProjectView::already_exists(name),
            ProjectResponse::WroteExport(path) => ProjectView::exported(path),
            ProjectResponse::Imported(result) => ProjectView::imported(result),
            ProjectResponse::Replayed(result) => ProjectView::replayed(result),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
