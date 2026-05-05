//! Mailbox — the typed envelope and the channel handle that carries it.

use tokio::sync::mpsc;

use crate::*;

/// A bus message — a tier-typed scope paired with the event to dispatch.
///
/// `T` is the scope tier (`AtHost`, `AtProject`, `AtBookmark`). The tier
/// is encoded in the type so the receiving actor doesn't have to
/// re-derive it; the inner `Scope<T>` already carries the resources it
/// needs for that tier. The `event` field carries lifecycle: a fresh
/// emission travels as `Event::New(...)`, a post-append notification
/// travels as `Event::Stored(...)`, a foreign event arriving for
/// ingest travels as `Event::Import(...)`.
#[derive(Clone)]
pub struct Message<T> {
    pub scope: Scope<T>,
    pub event: Event,
}

impl<T> Message<T> {
    pub fn new(scope: Scope<T>, event: impl Into<Event>) -> Self {
        Self {
            scope,
            event: event.into(),
        }
    }
}

/// Lifecycle wrapper for actor mailboxes.
///
/// Every per-actor mailbox carries `LifecycleMessage<DomainMessage>`.
/// The `Domain` variant carries the actor's regular work; the `Reset`
/// variant is a control signal — the actor wipes its state and runs
/// catch-up from history. Catch-up on initial spawn is implicit;
/// `Reset` is the explicit way to ask for a rebuild after the actor
/// is already running.
#[derive(Clone)]
pub enum LifecycleMessage<M> {
    Domain(M),
    Reset,
}

/// The over-the-wire form of a bus message — variants index by tier.
///
/// One channel feeds the host actor; the host actor matches on this
/// enum to decide whether to handle the message itself or to forward
/// it down the actor tree. New tiers are additive variants, never new
/// channels.
///
/// `Import` is bookmark-tier but routes to a separate `InboundActor`,
/// not the project actor — foreign events have their own ingestion
/// path (insert-or-ignore by id), distinct from the local `New` path
/// (append + assign rowid).
#[derive(Clone)]
pub enum RoutedMessage {
    Host(Message<AtHost>),
    Project(Message<AtProject>),
    Bookmark(Message<AtBookmark>),
    Import(Message<AtBookmark>),
}

impl From<Message<AtHost>> for RoutedMessage {
    fn from(message: Message<AtHost>) -> Self {
        Self::Host(message)
    }
}

impl From<Message<AtProject>> for RoutedMessage {
    fn from(message: Message<AtProject>) -> Self {
        Self::Project(message)
    }
}

impl From<Message<AtBookmark>> for RoutedMessage {
    fn from(message: Message<AtBookmark>) -> Self {
        // `Import` is dispatched explicitly by the bridge — the default
        // wrap for a bookmark message is `Bookmark` (local path).
        Self::Bookmark(message)
    }
}

/// The bus handle held by the HTTP server and threaded into services.
///
/// Cloneable, fire-and-forget. `tell` enqueues a message and returns
/// immediately; downstream actors react on their own tasks. A failed
/// send (receiver dropped) is logged at warn-level — the bus is
/// supposed to outlive every caller, so a closed receiver indicates
/// the server is tearing down.
#[derive(Clone)]
pub struct Mailbox {
    tx: mpsc::UnboundedSender<RoutedMessage>,
}

impl Mailbox {
    /// Create a new bus channel and return the sending handle alongside
    /// the receiver. The receiver is consumed by the host actor.
    pub fn open() -> (Self, mpsc::UnboundedReceiver<RoutedMessage>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, rx)
    }

    /// Fire-and-forget enqueue. `Into<RoutedMessage>` lets callers pass
    /// `Message<AtHost>`, `Message<AtProject>`, or `Message<AtBookmark>`
    /// directly without wrapping at the call site.
    pub fn tell(&self, message: impl Into<RoutedMessage>) {
        if let Err(err) = self.tx.send(message.into()) {
            tracing::warn!(error = %err, "bus receiver closed; message dropped");
        }
    }

    /// Fire-and-forget enqueue for foreign event ingestion. Routes to
    /// the inbound actor for the brain on the message's scope.
    pub fn tell_import(&self, message: Message<AtBookmark>) {
        if let Err(err) = self.tx.send(RoutedMessage::Import(message)) {
            tracing::warn!(error = %err, "bus receiver closed; import dropped");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn empty_event() -> Event {
        // A NewEvent with no domain content — we only care that the
        // envelope round-trips, not what's inside it.
        NewEvent::builder().data(noop_events()).build().into()
    }

    fn noop_events() -> Events {
        Events::Tenant(TenantEvents::TenantCreated(
            TenantCreated::builder_v1()
                .tenant(Tenant::builder().name(TenantName::new("noop")).build())
                .build()
                .into(),
        ))
    }

    fn host_scope(dir: &TempDir) -> Scope<AtHost> {
        let config = Config::builder().data_dir(dir.path().to_path_buf()).build();
        let host_infra = HostInfra {
            data_dir: dir.path().to_path_buf(),
            system_db_path: dir.path().join("system.db"),
            host_key_path: dir.path().join("host.key"),
            projects: Default::default(),
        };
        Scope::empty()
            .load(config)
            .to_host(std::sync::Arc::new(host_infra))
    }

    #[tokio::test]
    async fn mailbox_round_trips_a_host_message() {
        let dir = TempDir::new().unwrap();
        let (mailbox, mut rx) = Mailbox::open();

        mailbox.tell(Message::new(host_scope(&dir), empty_event()));

        match rx.recv().await {
            Some(RoutedMessage::Host(_)) => {}
            other => panic!("expected Host, got {:?}", other.is_some()),
        }
    }

    #[tokio::test]
    async fn mailbox_drops_silently_when_receiver_is_gone() {
        let dir = TempDir::new().unwrap();
        let (mailbox, rx) = Mailbox::open();
        drop(rx);

        // No panic, no error returned — fire-and-forget contract.
        mailbox.tell(Message::new(host_scope(&dir), empty_event()));
    }
}
