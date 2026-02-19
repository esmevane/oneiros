mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListConnectionsOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListConnections {
    /// Filter by nature.
    #[arg(long)]
    nature: Option<NatureName>,

    /// Filter by link (returns connections where this link is either from or to).
    #[arg(long)]
    link: Option<Link>,
}

impl ListConnections {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListConnectionsOutcomes>, ConnectionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let connections = client
            .list_connections(
                &context.ticket_token()?,
                self.nature.as_ref(),
                self.link.as_ref(),
            )
            .await?;

        if connections.is_empty() {
            outcomes.emit(ListConnectionsOutcomes::NoConnections);
        } else {
            outcomes.emit(ListConnectionsOutcomes::Connections(
                outcomes::ConnectionList(connections),
            ));
        }

        Ok(outcomes)
    }
}
