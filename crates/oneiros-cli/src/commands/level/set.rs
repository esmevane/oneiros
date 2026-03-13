use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetLevelOutcomes {
    #[outcome(message("Level '{0}' set."))]
    LevelSet(LevelName),
}

#[derive(Clone, Args, bon::Builder)]
pub struct SetLevel {
    /// The level name (identity).
    #[builder(into)]
    pub name: LevelName,

    /// A human-readable description of the level's purpose.
    #[arg(long, default_value = "")]
    #[builder(into)]
    pub description: Description,

    /// Guidance text for agents when applying this retention level.
    #[arg(long, default_value = "")]
    #[builder(into)]
    pub prompt: Prompt,
}

impl SetLevel {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetLevelOutcomes>, LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let info: Level = client
            .set_level(
                &context.ticket_token()?,
                Level::init(
                    self.name.clone(),
                    self.description.clone(),
                    self.prompt.clone(),
                ),
            )
            .await?
            .data()?;
        outcomes.emit(SetLevelOutcomes::LevelSet(info.name.clone()));

        Ok(outcomes)
    }
}
