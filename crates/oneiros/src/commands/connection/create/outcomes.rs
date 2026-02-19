use oneiros_model::ConnectionId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateConnectionOutcomes {
    #[outcome(message("Connection created: {0}"))]
    ConnectionCreated(ConnectionId),
}
