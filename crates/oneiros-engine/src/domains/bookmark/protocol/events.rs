use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = BookmarkEventsType, display = "kebab-case")]
pub enum BookmarkEvents {
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
pub struct BookmarkCreated {
    pub brain: BrainName,
    pub name: BookmarkName,
}

/// Derivation — a new bookmark forked from an existing one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkForked {
    pub brain: BrainName,
    pub name: BookmarkName,
    pub from: BookmarkName,
}

/// Navigation — the active bookmark changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkSwitched {
    pub brain: BrainName,
    pub name: BookmarkName,
}

/// Convergence — one bookmark's changes merged into another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkMerged {
    pub brain: BrainName,
    pub source: BookmarkName,
    pub target: BookmarkName,
}

/// Distribution — a bookmark was shared. Records the ticket that was
/// minted and the actor who shared it. Does not carry the composed URI
/// because that's derivable from the ticket + current host identity and
/// shouldn't be frozen in the event log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkShared {
    pub brain: BrainName,
    pub bookmark: BookmarkName,
    pub ticket_id: TicketId,
    pub shared_by: ActorId,
}

/// Distribution — a follow advanced its checkpoint after a collect
/// operation. The checkpoint is the new position after applying the
/// received events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkCollected {
    pub follow_id: FollowId,
    pub checkpoint: Checkpoint,
    pub events_received: u64,
}

/// Distribution — a follow was removed. Only the remote binding is
/// severed; events already collected into the local bookmark stay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkUnfollowed {
    pub follow_id: FollowId,
    pub brain: BrainName,
    pub bookmark: BookmarkName,
}
