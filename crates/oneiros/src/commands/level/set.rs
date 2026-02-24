use clap::Args;
use oneiros_client::Client;
use oneiros_model::LevelName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetLevelOutcomes {
    #[outcome(message("Level '{0}' set."))]
    LevelSet(LevelName),
}

#[derive(Clone, Args)]
pub struct SetLevel {
    /// The level name (identity).
    pub name: LevelName,

    /// A human-readable description of the level's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// Guidance text for agents when applying this retention level.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl SetLevel {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetLevelOutcomes>, LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_level(
                &context.ticket_token()?,
                LevelRecord::init(
                    self.description.clone(),
                    self.prompt.clone(),
                    Level {
                        name: self.name.clone(),
                    },
                ),
            )
            .await?;
        outcomes.emit(SetLevelOutcomes::LevelSet(info.name.clone()));

        Ok(outcomes)
    }
}
