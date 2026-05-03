use tokio::sync::mpsc;

use crate::*;

pub enum ProjectorMessage {
    Apply { config: Config, event: StoredEvent },
}

#[derive(Clone)]
pub struct ProjectorMailbox {
    tx: mpsc::UnboundedSender<ProjectorMessage>,
}

impl ProjectorMailbox {
    pub fn tell(&self, config: Config, event: StoredEvent) {
        let _ = self.tx.send(ProjectorMessage::Apply { config, event });
    }
}

pub struct ProjectorActor {
    canons: CanonIndex,
}

impl ProjectorActor {
    pub fn spawn(canons: CanonIndex) -> ProjectorMailbox {
        let (tx, mut rx) = mpsc::unbounded_channel::<ProjectorMessage>();
        let actor = Self { canons };

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                actor.handle(msg);
            }
        });

        ProjectorMailbox { tx }
    }

    fn handle(&self, msg: ProjectorMessage) {
        match msg {
            ProjectorMessage::Apply { config, event } => {
                if let Err(err) = self.apply(&config, &event) {
                    tracing::error!(?err, sequence = event.sequence, "projector: apply failed");
                }
            }
        }
    }

    fn apply(&self, config: &Config, event: &StoredEvent) -> Result<(), EventError> {
        let entry = self.canons.brain_entry(&config.brain)?;
        let projections = Projections::project_with_pipeline(entry.pipeline);
        let db = config.bookmark_conn()?;
        projections.apply_brain(&db, event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mailbox_round_trips_a_message() {
        let canons = CanonIndex::new();
        let mailbox = ProjectorActor::spawn(canons);

        let stored = StoredEvent::builder()
            .id(EventId::new())
            .sequence(1)
            .data(Event::default())
            .source(Source::default())
            .created_at(Timestamp::now())
            .build();

        mailbox.tell(Config::default(), stored);

        // Give the spawned task a chance to drain.
        tokio::task::yield_now().await;
    }
}
