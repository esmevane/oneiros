use clap::Args;
use oneiros_client::Client;
use oneiros_model::Nature;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListNaturesOutcomes {
    #[outcome(message("No natures configured."))]
    NoNatures,

    #[outcome(message("Natures: {0:?}"))]
    Natures(Vec<Nature>),
}

#[derive(Clone, Args)]
pub struct ListNatures;

impl ListNatures {
    pub async fn run(
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
