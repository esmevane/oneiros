use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ContinuityCommands {
    Wake(WakeAgent),
    Dream(DreamAgent),
    Introspect(IntrospectAgent),
    Reflect(ReflectAgent),
    Sense(SenseContent),
    Sleep(SleepAgent),
    Guidebook(GuidebookAgent),
    Emerge(EmergeAgent),
    Recede(RecedeAgent),
    Status(StatusAgent),
}

impl ContinuityCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, ContinuityError> {
        let client = context.client();
        let continuity_client = ContinuityClient::new(&client);

        let result = match self {
            ContinuityCommands::Wake(wake) => {
                ContinuityPresenter::new(continuity_client.wake(&wake.agent).await?).render()
            }
            ContinuityCommands::Dream(dream) => {
                ContinuityPresenter::new(continuity_client.dream(&dream.agent).await?).render()
            }
            ContinuityCommands::Introspect(introspect) => {
                ContinuityPresenter::new(continuity_client.introspect(&introspect.agent).await?)
                    .render()
            }
            ContinuityCommands::Reflect(reflect) => {
                ContinuityPresenter::new(continuity_client.reflect(&reflect.agent).await?).render()
            }
            ContinuityCommands::Sense(sense) => ContinuityPresenter::new(
                continuity_client
                    .sense(&sense.agent, sense.content.clone())
                    .await?,
            )
            .render(),
            ContinuityCommands::Sleep(sleep) => {
                ContinuityPresenter::new(continuity_client.sleep(&sleep.agent).await?).render()
            }
            ContinuityCommands::Guidebook(guidebook) => {
                ContinuityPresenter::new(continuity_client.guidebook(&guidebook.agent).await?)
                    .render()
            }
            ContinuityCommands::Emerge(emerge) => ContinuityPresenter::new(
                continuity_client
                    .emerge(
                        emerge.name.clone(),
                        emerge.persona.clone(),
                        emerge.description.clone(),
                    )
                    .await?,
            )
            .render(),
            ContinuityCommands::Recede(recede) => {
                ContinuityPresenter::new(continuity_client.recede(&recede.agent).await?).render()
            }
            ContinuityCommands::Status(status) => {
                ContinuityPresenter::new(continuity_client.status(&status.agent).await?).render()
            }
        };
        Ok(result)
    }
}
