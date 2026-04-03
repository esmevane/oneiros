use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum NatureCommands {
    Set(SetNature),
    Show(GetNature),
    List(ListNatures),
    Remove(RemoveNature),
}

impl NatureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, NatureError> {
        let client = context.client();
        let nature_client = NatureClient::new(&client);

        let response = match self {
            NatureCommands::Set(set) => nature_client.set(set).await?,
            NatureCommands::Show(get) => nature_client.get(&get.name).await?,
            NatureCommands::List(list) => nature_client.list(list).await?,
            NatureCommands::Remove(removal) => nature_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            NatureResponse::NatureSet(name) => format!("Nature '{name}' set."),
            NatureResponse::NatureDetails(wrapped) => {
                format!(
                    "Nature '{}'\n  Description: {}\n  Prompt: {}",
                    wrapped.data.name, wrapped.data.description, wrapped.data.prompt
                )
            }
            NatureResponse::Natures(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    out.push_str(&format!(
                        "  {} — {}\n\n",
                        wrapped.data.name, wrapped.data.description,
                    ));
                }
                out
            }
            NatureResponse::NoNatures => "No natures configured.".to_string(),
            NatureResponse::NatureRemoved(name) => format!("Nature '{name}' removed."),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
