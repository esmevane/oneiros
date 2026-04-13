use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum NatureCommands {
    Set(SetNature),
    Show(GetNature),
    List(ListNatures),
    Remove(RemoveNature),
}

impl NatureCommands {
    pub(crate) async fn execute(
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
            NatureResponse::NatureSet(name) => NatureView::confirmed("set", name).to_string(),
            NatureResponse::NatureDetails(wrapped) => NatureView::detail(&wrapped.data).to_string(),
            NatureResponse::Natures(listed) => {
                let table = NatureView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            NatureResponse::NoNatures => format!("{}", "No natures configured.".muted()),
            NatureResponse::NatureRemoved(name) => {
                NatureView::confirmed("removed", name).to_string()
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
