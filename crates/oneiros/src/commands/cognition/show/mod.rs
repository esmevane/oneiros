mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowCognitionOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowCognition {
    /// The cognition ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl ShowCognition {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowCognitionOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => CognitionId(id),
            None => {
                let all = client.list_cognitions(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|c| c.id.0.clone()).collect();
                CognitionId(self.id.resolve(&ids)?)
            }
        };

        let cognition = client.get_cognition(&token, &id).await?;
        outcomes.emit(ShowCognitionOutcomes::CognitionDetails(cognition));

        Ok(outcomes)
    }
}
