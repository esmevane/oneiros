use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum SensationCommands {
    Set(SetSensation),
    Show(GetSensation),
    List(ListSensations),
    Remove(RemoveSensation),
}

impl SensationCommands {
    pub(crate) async fn execute(
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
            SensationResponse::SensationSet(name) => {
                SensationView::confirmed("set", name).to_string()
            }
            SensationResponse::SensationDetails(wrapped) => {
                SensationView::detail(&wrapped.data).to_string()
            }
            SensationResponse::Sensations(listed) => {
                let table = SensationView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            SensationResponse::NoSensations => format!("{}", "No sensations configured.".muted()),
            SensationResponse::SensationRemoved(name) => {
                SensationView::confirmed("removed", name).to_string()
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
