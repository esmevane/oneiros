use crate::*;

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
// #[deprecated = "use `Message`"]
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
