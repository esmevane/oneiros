use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct CognitionDetail(pub Cognition);

impl core::fmt::Display for CognitionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_detail())
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowCognitionOutcomes {
    #[outcome(
        message("{0}"),
        prompt(
            "Does this connect to something? Trace it with `oneiros experience create <agent> <sensation> <description>`."
        )
    )]
    CognitionDetails(CognitionDetail),
}

#[derive(Clone, Args)]
pub struct ShowCognition {
    /// The cognition ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,
}

impl ShowCognition {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowCognitionOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => CognitionId(id),
            None => {
                let all = client.list_cognitions(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|c| c.id.0).collect();
                CognitionId(self.id.resolve(&ids)?)
            }
        };

        let cognition = client.get_cognition(&token, &id).await?;
        outcomes.emit(ShowCognitionOutcomes::CognitionDetails(CognitionDetail(
            cognition,
        )));

        Ok(outcomes)
    }
}
