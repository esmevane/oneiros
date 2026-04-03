use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SensationCommands {
    Set(SetSensation),
    Show(GetSensation),
    List(ListSensations),
    Remove(RemoveSensation),
}

impl SensationCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, SensationError> {
        let client = context.client();
        let sensation_client = SensationClient::new(&client);

        let response = match self {
            SensationCommands::Set(set) => sensation_client.set(set).await?,
            SensationCommands::Show(get) => sensation_client.get(&get.name).await?,
            SensationCommands::List(list) => sensation_client.list(list).await?,
            SensationCommands::Remove(removal) => sensation_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            SensationResponse::SensationSet(name) => format!("Sensation '{name}' set."),
            SensationResponse::SensationDetails(wrapped) => {
                format!(
                    "Sensation '{}'\n  Description: {}\n  Prompt: {}",
                    wrapped.data.name, wrapped.data.description, wrapped.data.prompt
                )
            }
            SensationResponse::Sensations(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    out.push_str(&format!(
                        "  {} — {}\n\n",
                        wrapped.data.name, wrapped.data.description,
                    ));
                }
                out
            }
            SensationResponse::NoSensations => "No sensations configured.".to_string(),
            SensationResponse::SensationRemoved(name) => format!("Sensation '{name}' removed."),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
