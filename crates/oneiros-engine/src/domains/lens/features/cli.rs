use clap::Subcommand;

use crate::*;

/// CLI subcommands for the lens domain. Parse and Explain are pure
/// inspection operations on the lens query language — they parse the
/// source, optionally validate against the registry, and return the
/// result tree as text for terminal display.
#[derive(Debug, Subcommand)]
pub(crate) enum LensCommands {
    Parse(ParseLens),
    Explain(ExplainLens),
}

impl LensCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, LensError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Parse(req) => req.execute_request(&client).await?,
            Self::Explain(req) => req.execute_request(&client).await?,
        };

        let response: LensResponse = serde_json::from_slice(&bytes)?;
        Ok(LensView::new(response).render().map(Into::into))
    }
}
