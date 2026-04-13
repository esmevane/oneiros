use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = BookmarkEventsType, display = "kebab-case")]
pub(crate) enum BookmarkEvents {
    BookmarkCreated(BookmarkCreated),
    BookmarkForked(BookmarkForked),
    BookmarkSwitched(BookmarkSwitched),
    BookmarkMerged(BookmarkMerged),
    /// Distribution — a bookmark was shared, minting a ticket for it.
    BookmarkShared(BookmarkShared),
    /// Distribution — a local bookmark begins following a source (local
    /// ref or peer link). Carries the full Follow record.
    BookmarkFollowed(Follow),
    /// Distribution — a follow advanced its checkpoint after a collect.
    BookmarkCollected(BookmarkCollected),
    /// Distribution — a follow was removed. Its last-collected events
    /// remain in the local bookmark; only the remote binding is severed.
    BookmarkUnfollowed(BookmarkUnfollowed),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (BookmarkEventsType::BookmarkCreated, "bookmark-created"),
            (BookmarkEventsType::BookmarkForked, "bookmark-forked"),
            (BookmarkEventsType::BookmarkSwitched, "bookmark-switched"),
            (BookmarkEventsType::BookmarkMerged, "bookmark-merged"),
            (BookmarkEventsType::BookmarkShared, "bookmark-shared"),
            (BookmarkEventsType::BookmarkFollowed, "bookmark-followed"),
            (BookmarkEventsType::BookmarkCollected, "bookmark-collected"),
            (
                BookmarkEventsType::BookmarkUnfollowed,
                "bookmark-unfollowed",
            ),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }
}

/// Genesis — a bookmark comes into existence (e.g. "main" at brain init).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkCreated {
    pub(crate) brain: BrainName,
    pub(crate) name: BookmarkName,
}

/// Derivation — a new bookmark forked from an existing one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkForked {
    pub(crate) brain: BrainName,
    pub(crate) name: BookmarkName,
    pub(crate) from: BookmarkName,
}

/// Navigation — the active bookmark changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkSwitched {
    pub(crate) brain: BrainName,
    pub(crate) name: BookmarkName,
}

/// Convergence — one bookmark's changes merged into another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkMerged {
    pub(crate) brain: BrainName,
    pub(crate) source: BookmarkName,
    pub(crate) target: BookmarkName,
}

/// Distribution — a bookmark was shared. Records the ticket that was
/// minted and the actor who shared it. Does not carry the composed URI
/// because that's derivable from the ticket + current host identity and
/// shouldn't be frozen in the event log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkShared {
    pub(crate) brain: BrainName,
    pub(crate) bookmark: BookmarkName,
    pub(crate) ticket_id: TicketId,
    pub(crate) shared_by: ActorId,
}

/// Distribution — a follow advanced its checkpoint after a collect
/// operation. The checkpoint is the new position after applying the
/// received events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkCollected {
    pub(crate) follow_id: FollowId,
    pub(crate) checkpoint: Checkpoint,
    pub(crate) events_received: u64,
}

/// Distribution — a follow was removed. Only the remote binding is
/// severed; events already collected into the local bookmark stay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BookmarkUnfollowed {
    pub(crate) follow_id: FollowId,
    pub(crate) brain: BrainName,
    pub(crate) bookmark: BookmarkName,
}
