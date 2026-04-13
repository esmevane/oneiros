use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ContinuityCommands {
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
    pub(crate) async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, ContinuityError> {
        let client = context.client();
        let continuity_client = ContinuityClient::new(&client);

        let (result, deep) = match self {
            ContinuityCommands::Wake(wake) => {
                (continuity_client.wake(&wake.agent).await?, wake.deep)
            }
            ContinuityCommands::Dream(dream) => {
                (continuity_client.dream(&dream.agent).await?, dream.deep)
            }
            ContinuityCommands::Introspect(introspect) => (
                continuity_client.introspect(&introspect.agent).await?,
                false,
            ),
            ContinuityCommands::Reflect(reflect) => {
                (continuity_client.reflect(&reflect.agent).await?, false)
            }
            ContinuityCommands::Sense(sense) => (continuity_client.sense(sense).await?, false),
            ContinuityCommands::Sleep(sleep) => {
                (continuity_client.sleep(&sleep.agent).await?, false)
            }
            ContinuityCommands::Guidebook(guidebook) => {
                (continuity_client.guidebook(&guidebook.agent).await?, false)
            }
            ContinuityCommands::Emerge(emerge) => (continuity_client.emerge(emerge).await?, false),
            ContinuityCommands::Recede(recede) => {
                (continuity_client.recede(&recede.agent).await?, false)
            }
            ContinuityCommands::Status(_) => (continuity_client.status().await?, false),
        };

        Ok(ContinuityView::new(result).with_deep(deep).render())
    }
}
