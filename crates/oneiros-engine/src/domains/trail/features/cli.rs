use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum TrailCommands {
    /// Events that touched an entity
    Of(TrailOf),
    /// Entities emitted by an event
    From(TrailFrom),
}

impl TrailCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, TrailError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Of(of) => of.execute_request(&client).await?,
            Self::From(from) => from.execute_request(&client).await?,
        };

        let response: TrailResponse = serde_json::from_slice(&bytes)?;
        Ok(TrailView::new(response).render().map(Into::into))
    }
}
