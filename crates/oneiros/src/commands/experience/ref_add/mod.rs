mod outcomes;

use clap::Args;
use oneiros_client::{AddExperienceRefRequest, Client};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RefAddOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RefAdd {
    /// The experience ID to add a reference to.
    experience_id: ExperienceId,

    /// The ID of the record to reference.
    record_id: Id,

    /// The kind of record being referenced.
    record_kind: RecordKind,

    /// Optional role label for this reference.
    #[arg(long)]
    role: Option<String>,
}

impl RefAdd {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RefAddOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let experience = client
            .add_experience_ref(
                &context.ticket_token()?,
                &self.experience_id,
                AddExperienceRefRequest {
                    record_id: self.record_id,
                    record_kind: self.record_kind.clone(),
                    role: self.role.as_ref().map(Label::new),
                },
            )
            .await?;
        outcomes.emit(RefAddOutcomes::RefAdded(experience.id));

        Ok(outcomes)
    }
}
