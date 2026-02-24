use clap::Args;
use oneiros_client::Client;
use oneiros_link::Link;
use oneiros_model::{Connection, ConnectionId, Identity};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ConnectionList(pub Vec<Identity<ConnectionId, Connection>>);

impl core::fmt::Display for ConnectionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|connection| format!("{connection}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListConnectionsOutcomes {
    #[outcome(message("No connections found."))]
    NoConnections,

    #[outcome(message("{0}"))]
    Connections(ConnectionList),
}

#[derive(Clone, Args)]
pub struct ListConnections {
    /// Filter by nature.
    #[arg(long)]
    nature: Option<NatureName>,

    /// Filter by link (returns connections where this link is either from or to).
    #[arg(long)]
    link: Option<Link>,
}

impl ListConnections {
    pub async fn run(
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
            outcomes.emit(ListConnectionsOutcomes::Connections(ConnectionList(
                connections,
            )));
        }

        Ok(outcomes)
    }
}
