mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::SetLevelOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetLevel {
    /// The level name (identity).
    name: LevelName,

    /// A human-readable description of the level's purpose.
    #[arg(long, default_value = "")]
    description: Description,

    /// Guidance text for agents when applying this retention level.
    #[arg(long, default_value = "")]
    prompt: Prompt,
}

impl SetLevel {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<SetLevelOutcomes>, LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_level(
                &context.ticket_token()?,
                Level {
                    name: self.name.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(SetLevelOutcomes::LevelSet(info.name));

        Ok(outcomes)
    }
}
