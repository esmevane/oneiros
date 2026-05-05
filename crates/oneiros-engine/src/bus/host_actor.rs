//! `HostActor` — the static, always-running root of the actor tree.
//!
//! Receives every `RoutedMessage` from the bus. Host-tier work
//! (appending to the host event log, applying system-tier projections)
//! happens here. Project- and bookmark-tier `New` messages are
//! forwarded to lazily-spawned `ProjectActor` children, one per brain.
//! `Import` messages are forwarded to lazily-spawned `InboundActor`
//! children — sibling to the project actor — that handle foreign
//! event ingestion. The brain is read off the message's scope; first
//! sighting spawns the child.

use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::*;

pub struct HostActor {
    /// Server config — used to compose project-tier scopes when
    /// lazy-spawning project children.
    config: Config,
    canons: CanonIndex,
    projects: HashMap<BrainName, ProjectMailbox>,
    inbounds: HashMap<BrainName, InboundMailbox>,
}

impl HostActor {
    /// Spawn the host actor in the background. Returns a `JoinHandle` so
    /// callers can hold it for the server's lifetime; dropping the handle
    /// detaches the task (the actor still runs until the receiver closes).
    pub fn spawn(
        config: Config,
        canons: CanonIndex,
        rx: mpsc::UnboundedReceiver<RoutedMessage>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let actor = Self {
                config,
                canons,
                projects: HashMap::new(),
                inbounds: HashMap::new(),
            };
            actor.run(rx).await;
        })
    }

    async fn run(mut self, mut rx: mpsc::UnboundedReceiver<RoutedMessage>) {
        while let Some(message) = rx.recv().await {
            match message {
                RoutedMessage::Host(message) => {
                    if let Err(err) = self.handle_host(message).await {
                        tracing::error!(?err, "host actor: handle failed");
                    }
                }
                RoutedMessage::Project(message) => {
                    let brain = message.scope.project().name.clone();
                    match self.project_mailbox(brain) {
                        Ok(mailbox) => mailbox.tell_domain(ProjectDomain::Project(message)),
                        Err(err) => tracing::error!(?err, "host actor: project compose failed"),
                    }
                }
                RoutedMessage::Bookmark(message) => {
                    let brain = message.scope.project().name.clone();
                    match self.project_mailbox(brain) {
                        Ok(mailbox) => mailbox.tell_domain(ProjectDomain::Bookmark(message)),
                        Err(err) => tracing::error!(?err, "host actor: project compose failed"),
                    }
                }
                RoutedMessage::Import(message) => {
                    let brain = message.scope.project().name.clone();
                    match self.inbound_mailbox(brain) {
                        Ok(mailbox) => mailbox.tell_domain(message),
                        Err(err) => tracing::error!(?err, "host actor: inbound compose failed"),
                    }
                }
            }
        }

        tracing::info!("host actor: bus closed, exiting");
    }

    async fn handle_host(&self, message: Message<AtHost>) -> Result<(), EventError> {
        let Message { scope, event } = message;

        let new_event = match event {
            Event::New(boxed) => *boxed,
            // Stored events at the host tier are notifications for
            // downstream actors (chronicle, future host-tier consumers).
            // The host actor doesn't have downstream consumers yet, so
            // it's a no-op for now.
            Event::Stored(_) => return Ok(()),
            // The other variants don't make sense as bus traffic — they
            // arise from parsing on-disk rows. Ignored at the host
            // boundary.
            Event::Known(_)
            | Event::Ephemeral(_)
            | Event::Unknown(_)
            | Event::Malformed
            | Event::Import(_) => {
                tracing::warn!(
                    event_type = event.event_type(),
                    "host actor: unexpected event variant on bus"
                );
                return Ok(());
            }
        };

        let host_db = HostDb::open(&scope).await?;
        let projections = Projections::system();
        projections.migrate(&host_db)?;
        let stored = EventLog::new(&host_db).append(&new_event)?;
        projections.apply(&host_db, &stored)?;

        Ok(())
    }

    fn project_mailbox(&mut self, brain: BrainName) -> Result<ProjectMailbox, ComposeError> {
        if let Some(mb) = self.projects.get(&brain) {
            return Ok(mb.clone());
        }
        let scope = ComposeScope::new(self.config.clone()).project(brain.clone())?;
        let mb = ProjectActor::spawn(brain.clone(), scope, self.canons.clone());
        self.projects.insert(brain, mb.clone());
        Ok(mb)
    }

    fn inbound_mailbox(&mut self, brain: BrainName) -> Result<InboundMailbox, ComposeError> {
        if let Some(mb) = self.inbounds.get(&brain) {
            return Ok(mb.clone());
        }
        // The inbound actor needs a way to forward Stored events into
        // the project actor's `Forward` path — so the same downstream
        // (bookmark + chronicle) sees them as if they were locally
        // stored. We give it a clone of the project actor's mailbox.
        let project = self.project_mailbox(brain.clone())?;
        let scope = ComposeScope::new(self.config.clone()).project(brain.clone())?;
        let mb = InboundActor::spawn(brain.clone(), scope, project);
        self.inbounds.insert(brain, mb.clone());
        Ok(mb)
    }
}
