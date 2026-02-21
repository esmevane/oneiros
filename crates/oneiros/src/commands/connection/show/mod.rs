mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowConnectionOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowConnection {
    /// The connection ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl ShowConnection {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowConnectionOutcomes>, ConnectionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ConnectionId(id),
            None => {
                let all = client.list_connections(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|c| c.id.inner().clone()).collect();
                ConnectionId(self.id.resolve(&ids)?)
            }
        };

        let connection = client.get_connection(&token, &id).await?;

        outcomes.emit(ShowConnectionOutcomes::ConnectionDetails(
            outcomes::ConnectionDetail(connection),
        ));

        Ok(outcomes)
    }
}
