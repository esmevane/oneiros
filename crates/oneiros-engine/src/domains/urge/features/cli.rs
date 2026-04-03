use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum UrgeCommands {
    Set(SetUrge),
    Show(GetUrge),
    List(ListUrges),
    Remove(RemoveUrge),
}

impl UrgeCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, UrgeError> {
        let client = context.client();
        let urge_client = UrgeClient::new(&client);

        let response = match self {
            UrgeCommands::Set(set) => urge_client.set(set).await?,
            UrgeCommands::Show(get) => urge_client.get(&get.name).await?,
            UrgeCommands::List(list) => urge_client.list(list).await?,
            UrgeCommands::Remove(removal) => urge_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            UrgeResponse::UrgeSet(name) => format!("Urge '{name}' set."),
            UrgeResponse::UrgeDetails(u) => {
                format!(
                    "Urge '{}'\n  Description: {}\n  Prompt: {}",
                    u.name, u.description, u.prompt
                )
            }
            UrgeResponse::Urges(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for urge in &listed.items {
                    out.push_str(&format!("  {} — {}\n\n", urge.name, urge.description,));
                }
                out
            }
            UrgeResponse::NoUrges => "No urges configured.".to_string(),
            UrgeResponse::UrgeRemoved(name) => format!("Urge '{name}' removed."),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
