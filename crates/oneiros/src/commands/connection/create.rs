use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct ConnectionCreatedResult {
    pub id: ConnectionId,
    #[serde(skip)]
    pub ref_token: RefToken,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateConnectionOutcomes {
    #[outcome(message("Connection created: {}", .0.ref_token))]
    ConnectionCreated(ConnectionCreatedResult),
}

#[derive(Clone, Args)]
pub struct CreateConnection {
    /// The nature of the connection (must already exist).
    nature: NatureName,

    /// The source entity ref (ref:base64url-encoded).
    from_ref: RefToken,

    /// The target entity ref (ref:base64url-encoded).
    to_ref: RefToken,
}

impl CreateConnection {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<CreateConnectionOutcomes>, ConnectionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let connection = client
            .create_connection(
                &context.ticket_token()?,
                CreateConnectionRequest {
                    nature: self.nature.clone(),
                    from_ref: self.from_ref.clone().into_inner(),
                    to_ref: self.to_ref.clone().into_inner(),
                },
            )
            .await?;

        let ref_token = connection.ref_token();

        outcomes.emit(CreateConnectionOutcomes::ConnectionCreated(
            ConnectionCreatedResult {
                id: connection.id,
                ref_token,
            },
        ));

        Ok(outcomes)
    }
}
