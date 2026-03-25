use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    Init {
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long, short)]
        yes: bool,
    },
}

impl SystemCommands {
    pub async fn execute(&self, ctx: &SystemContext) -> Result<Rendered<Responses>, SystemError> {
        let response = match self {
            SystemCommands::Init { name, .. } => {
                let name = name.clone().unwrap_or_else(|| "onerios user".to_string());
                SystemService::init(ctx, name).await?
            }
        };

        let prompt = match &response {
            SystemResponse::SystemInitialized(name) => {
                format!("System initialized for '{name}'.")
            }
            SystemResponse::HostAlreadyInitialized => "System already initialized.".to_string(),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
