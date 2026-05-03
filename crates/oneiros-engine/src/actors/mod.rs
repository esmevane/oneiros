mod projector;

pub use projector::*;

use crate::*;

/// A bundle of mailboxes — one per downstream actor — held by the contexts
/// that publish events (`ProjectLog`, etc.). Sending through a mailbox is
/// fire-and-forget; actors react asynchronously on their own tasks.
#[derive(Clone)]
pub struct Mailboxes {
    pub projector: ProjectorMailbox,
}

impl Mailboxes {
    pub fn spawn(canons: CanonIndex) -> Self {
        Self {
            projector: ProjectorActor::spawn(canons),
        }
    }
}
