use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    Create {
        nature: String,
        from_ref: String,
        to_ref: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        nature: Option<String>,
        #[arg(long)]
        entity_ref: Option<String>,
    },
    Remove {
        id: String,
    },
}

impl ConnectionCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, ConnectionError> {
        let result = match self {
            ConnectionCommands::Create {
                nature,
                from_ref,
                to_ref,
            } => ConnectionService::create(
                context,
                from_ref.clone(),
                to_ref.clone(),
                nature.clone(),
            )?
            .into(),

            ConnectionCommands::Show { id } => {
                let id: ConnectionId = id.parse()?;
                ConnectionService::get(context, &id)?.into()
            }
            ConnectionCommands::List { entity_ref, .. } => {
                ConnectionService::list(context, entity_ref.as_deref())?.into()
            }
            ConnectionCommands::Remove { id } => {
                let id: ConnectionId = id.parse()?;
                ConnectionService::remove(context, &id)?.into()
            }
        };
        Ok(result)
    }
}
