mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListNaturesOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListNatures;

impl ListNatures {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListNaturesOutcomes>, NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let natures = client.list_natures(&context.ticket_token()?).await?;

        if natures.is_empty() {
            outcomes.emit(ListNaturesOutcomes::NoNatures);
        } else {
            outcomes.emit(ListNaturesOutcomes::Natures(natures));
        }

        Ok(outcomes)
    }
}
