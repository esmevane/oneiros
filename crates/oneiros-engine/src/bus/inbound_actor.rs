//! `InboundActor` — handles foreign event ingestion for one brain.
//!
//! Sibling to the `ProjectActor` under `HostActor`, lazy-spawned per
//! brain. Listens to `Event::Import(StoredEvent)` messages dispatched
//! by the bridge during peer collect (or by any future ingestion
//! source).
//!
//! Its job:
//! 1. Insert-or-ignore the event into the project's events.db.
//!    Idempotent on event id.
//! 2. Tell the brain's project actor "Forward" — same downstream as a
//!    local emission, but no append happens.
//!
//! After this, the bookmark + chronicle children see a regular
//! `Stored` notification and project / record it like any other
//! event. Origin is the inbound actor's concern only; downstream
//! doesn't know or care where the event came from.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub struct InboundMailbox {
    tx: mpsc::UnboundedSender<LifecycleMessage<Message<AtBookmark>>>,
}

impl InboundMailbox {
    pub fn tell(&self, message: LifecycleMessage<Message<AtBookmark>>) {
        if let Err(err) = self.tx.send(message) {
            tracing::warn!(error = %err, "inbound actor: receiver closed; message dropped");
        }
    }

    pub fn tell_domain(&self, message: Message<AtBookmark>) {
        self.tell(LifecycleMessage::Domain(message));
    }
}

pub struct InboundActor {
    brain: BrainName,
    scope: Scope<AtProject>,
    project: ProjectMailbox,
}

impl InboundActor {
    pub fn spawn(
        brain: BrainName,
        scope: Scope<AtProject>,
        project: ProjectMailbox,
    ) -> InboundMailbox {
        let (tx, rx) = mpsc::unbounded_channel();
        let actor = Self {
            brain,
            scope,
            project,
        };
        tokio::spawn(actor.run(rx));
        InboundMailbox { tx }
    }

    async fn run(self, mut rx: mpsc::UnboundedReceiver<LifecycleMessage<Message<AtBookmark>>>) {
        while let Some(message) = rx.recv().await {
            match message {
                LifecycleMessage::Domain(message) => {
                    if let Err(err) = self.handle(message).await {
                        tracing::error!(
                            brain = %self.brain,
                            ?err,
                            "inbound actor: ingest failed",
                        );
                    }
                }
                LifecycleMessage::Reset => {
                    // Inbound actor has no state to reset — events.db
                    // is the project actor's concern, and projection
                    // state lives in the children. Ignored here.
                }
            }
        }
    }

    async fn handle(&self, message: Message<AtBookmark>) -> Result<(), EventError> {
        let Message { scope, event } = message;
        let stored = match event {
            Event::Import(boxed) => *boxed,
            // The actor only listens to Import — anything else is a
            // misroute.
            _ => return Ok(()),
        };
        // Step 1: ensure the event is in events.db. Insert-or-ignore
        // on id — duplicate ingests are no-ops.
        let events_db = EventsDb::open(&self.scope).await?;
        EventLog::new(&events_db).init()?;
        EventLog::new(&events_db).import(&stored)?;
        drop(events_db);

        // Step 2: notify the project actor. It owns the bookmark +
        // chronicle children, and forwards Stored to them as if the
        // event had been locally appended.
        let stored_message = Message {
            scope,
            event: Event::Stored(Box::new(stored)),
        };
        self.project
            .tell_domain(ProjectDomain::Forward(stored_message));

        Ok(())
    }
}
