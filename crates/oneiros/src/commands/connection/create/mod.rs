mod outcomes;

use clap::Args;
use oneiros_client::{Client, CreateConnectionRequest};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::CreateConnectionOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct CreateConnection {
    /// The nature of the connection (must already exist).
    nature: NatureName,

    /// The source link (base64url-encoded content address).
    from_link: Link,

    /// The target link (base64url-encoded content address).
    to_link: Link,
}

impl CreateConnection {
    pub(crate) async fn run(
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
                    from_link: self.from_link.clone(),
                    to_link: self.to_link.clone(),
                },
            )
            .await?;

        outcomes.emit(CreateConnectionOutcomes::ConnectionCreated(connection.id));

        Ok(outcomes)
    }
}
