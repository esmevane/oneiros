use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    Init(InitSystem),
}

impl SystemCommands {
    pub async fn execute(
        &self,
        context: SystemContext,
    ) -> Result<Rendered<Responses>, SystemError> {
        let response = match self {
            SystemCommands::Init(init) => SystemService::init(&context, init).await?,
        };

        Ok(SystemView::new(response).render().map(Into::into))
    }
}
