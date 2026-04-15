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

        let deep = match self {
            ContinuityCommands::Wake(wake) => wake.deep,
            ContinuityCommands::Dream(dream) => dream.deep,
            ContinuityCommands::Introspect(_)
            | ContinuityCommands::Reflect(_)
            | ContinuityCommands::Sleep(_)
            | ContinuityCommands::Sense(_)
            | ContinuityCommands::Guidebook(_)
            | ContinuityCommands::Emerge(_)
            | ContinuityCommands::Recede(_)
            | ContinuityCommands::Status(_) => false,
        };

        let result = match self {
            ContinuityCommands::Wake(wake) => continuity_client.wake(&wake.agent).await?,
            ContinuityCommands::Dream(dream) => continuity_client.dream(&dream.agent).await?,
            ContinuityCommands::Introspect(introspect) => {
                continuity_client.introspect(&introspect.agent).await?
            }
            ContinuityCommands::Reflect(reflect) => {
                continuity_client.reflect(&reflect.agent).await?
            }
            ContinuityCommands::Sense(sense) => continuity_client.sense(sense).await?,
            ContinuityCommands::Sleep(sleep) => continuity_client.sleep(&sleep.agent).await?,
            ContinuityCommands::Guidebook(guidebook) => {
                continuity_client.guidebook(&guidebook.agent).await?
            }
            ContinuityCommands::Emerge(emerge) => continuity_client.emerge(emerge).await?,
            ContinuityCommands::Recede(recede) => continuity_client.recede(&recede.agent).await?,
            ContinuityCommands::Status(_) => continuity_client.status().await?,
        };

        Ok(ContinuityView::new(result)
            .with_deep(deep)
            .render()
            .map(Into::into))
    }
}
