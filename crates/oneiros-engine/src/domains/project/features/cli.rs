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

        let prompt = match &response {
            ProjectResponse::Initialized(InitResult {
                brain_name,
                token: _,
            }) => format!("Brain '{brain_name}' created."),
            ProjectResponse::BrainAlreadyExists(name) => format!("Brain '{name}' already exists."),
            ProjectResponse::WroteExport(path) => {
                format!("Export written to '{path}'.")
            }
            ProjectResponse::Imported(ImportResult { imported, replayed }) => {
                format!("Imported {imported} events, replayed {replayed}.",)
            }
            ProjectResponse::Replayed(ReplayResult { replayed }) => {
                format!("Replayed {replayed} events.")
            }
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
