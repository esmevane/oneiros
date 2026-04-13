use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum UrgeCommands {
    Set(SetUrge),
    Show(GetUrge),
    List(ListUrges),
    Remove(RemoveUrge),
}

impl UrgeCommands {
    pub(crate) async fn execute(
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
            UrgeResponse::UrgeSet(name) => UrgeView::confirmed("set", name).to_string(),
            UrgeResponse::UrgeDetails(urge) => UrgeView::detail(urge).to_string(),
            UrgeResponse::Urges(listed) => {
                let table = UrgeView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            UrgeResponse::NoUrges => format!("{}", "No urges configured.".muted()),
            UrgeResponse::UrgeRemoved(name) => UrgeView::confirmed("removed", name).to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
