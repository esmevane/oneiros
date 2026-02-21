mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowMemoryOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowMemory {
    /// The memory ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl ShowMemory {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowMemoryOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => MemoryId(id),
            None => {
                let all = client.list_memories(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|m| m.id.inner().clone()).collect();
                MemoryId(self.id.resolve(&ids)?)
            }
        };

        let memory = client.get_memory(&token, &id).await?;
        outcomes.emit(ShowMemoryOutcomes::MemoryDetails(memory));

        Ok(outcomes)
    }
}
