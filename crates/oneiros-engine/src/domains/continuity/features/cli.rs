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
        config: &Config,
    ) -> Result<Rendered<Responses>, ContinuityError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Wake(wake) => wake.execute_request(&client).await?,
            Self::Dream(dream) => dream.execute_request(&client).await?,
            Self::Introspect(introspecting) => introspecting.execute_request(&client).await?,
            Self::Reflect(reflecting) => reflecting.execute_request(&client).await?,
            Self::Sense(sensing) => sensing.execute_request(&client).await?,
            Self::Sleep(sleeping) => sleeping.execute_request(&client).await?,
            Self::Guidebook(lookup) => lookup.execute_request(&client).await?,
            Self::Emerge(emerging) => emerging.execute_request(&client).await?,
            Self::Recede(receding) => receding.execute_request(&client).await?,
            Self::Status(status) => status.execute_request(&client).await?,
        };

        let response: ContinuityResponse = serde_json::from_slice(&bytes)?;
        Ok(ContinuityView::new(response).render().map(Into::into))
    }
}
