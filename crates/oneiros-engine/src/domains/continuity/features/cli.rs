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
                let WakeAgent::V1(wake) = wake;
                continuity_client.wake(&wake.agent).await?
            }
            ContinuityCommands::Dream(dream) => {
                let DreamAgent::V1(dream) = dream;
                continuity_client.dream(&dream.agent).await?
            }
            ContinuityCommands::Introspect(introspecting) => {
                let IntrospectAgent::V1(introspecting) = introspecting;
                continuity_client.introspect(&introspecting.agent).await?
            }
            ContinuityCommands::Reflect(reflecting) => {
                let ReflectAgent::V1(reflecting) = reflecting;
                continuity_client.reflect(&reflecting.agent).await?
            }
            ContinuityCommands::Sense(sensing) => continuity_client.sense(sensing).await?,
            ContinuityCommands::Sleep(sleeping) => {
                let SleepAgent::V1(sleeping) = sleeping;
                continuity_client.sleep(&sleeping.agent).await?
            }
            ContinuityCommands::Guidebook(lookup) => {
                let GuidebookAgent::V1(lookup) = lookup;
                continuity_client.guidebook(&lookup.agent).await?
            }
            ContinuityCommands::Emerge(emerging) => continuity_client.emerge(emerging).await?,
            ContinuityCommands::Recede(receding) => {
                let RecedeAgent::V1(receding) = receding;
                continuity_client.recede(&receding.agent).await?
            }
            ContinuityCommands::Status(_) => continuity_client.status().await?,
        };

        Ok(ContinuityView::new(result).render().map(Into::into))
    }
}
