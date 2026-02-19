use oneiros_model::ConnectionId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveConnectionOutcomes {
    #[outcome(message("Connection {0} removed."))]
    ConnectionRemoved(ConnectionId),
}
