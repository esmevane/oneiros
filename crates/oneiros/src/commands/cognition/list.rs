use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct CognitionList(pub Vec<Cognition>);

impl core::fmt::Display for CognitionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|cognition| format!("{cognition}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListCognitionsOutcomes {
    #[outcome(message("No cognitions found."))]
    NoCognitions,

    #[outcome(
        message("{0}"),
        prompt(
            "Which of these are still working threads? Consolidate what's crystallized with `oneiros memory add <agent>`."
        )
    )]
    Cognitions(CognitionList),
}

#[derive(Clone, Args)]
pub struct ListCognitions {
    /// Filter by agent name.
    #[arg(long)]
    agent: Option<AgentName>,

    /// Filter by texture name.
    #[arg(long)]
    texture: Option<TextureName>,
}

impl ListCognitions {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListCognitionsOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let cognitions = client
            .list_cognitions(
                &context.ticket_token()?,
                self.agent.as_ref(),
                self.texture.as_ref(),
            )
            .await?;

        if cognitions.is_empty() {
            outcomes.emit(ListCognitionsOutcomes::NoCognitions);
        } else {
            outcomes.emit(ListCognitionsOutcomes::Cognitions(CognitionList(
                cognitions,
            )));
        }

        Ok(outcomes)
    }
}
