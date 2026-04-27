use bon::Builder;
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
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum BookmarkCreated {
    Current(BookmarkCreatedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BookmarkCreatedV1 {
    pub brain: BrainName,
    pub name: BookmarkName,
}

impl BookmarkCreated {
    pub fn build_v1() -> BookmarkCreatedV1Builder {
        BookmarkCreatedV1::builder()
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn name(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.name,
        }
    }
}

/// Derivation — a new bookmark forked from an existing one.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum BookmarkForked {
    Current(BookmarkForkedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BookmarkForkedV1 {
    pub brain: BrainName,
    pub name: BookmarkName,
    pub from: BookmarkName,
}

impl BookmarkForked {
    pub fn build_v1() -> BookmarkForkedV1Builder {
        BookmarkForkedV1::builder()
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn name(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn from(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.from,
        }
    }
}

/// Navigation — the active bookmark changed.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum BookmarkSwitched {
    Current(BookmarkSwitchedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BookmarkSwitchedV1 {
    pub brain: BrainName,
    pub name: BookmarkName,
}

impl BookmarkSwitched {
    pub fn build_v1() -> BookmarkSwitchedV1Builder {
        BookmarkSwitchedV1::builder()
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn name(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.name,
        }
    }
}

/// Convergence — one bookmark's changes merged into another.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum BookmarkMerged {
    Current(BookmarkMergedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BookmarkMergedV1 {
    pub brain: BrainName,
    pub source: BookmarkName,
    pub target: BookmarkName,
}

impl BookmarkMerged {
    pub fn build_v1() -> BookmarkMergedV1Builder {
        BookmarkMergedV1::builder()
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn source(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.source,
        }
    }

    pub fn target(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.target,
        }
    }
}

/// Distribution — a bookmark was shared. Records the ticket that was
/// minted and the actor who shared it. Does not carry the composed URI
/// because that's derivable from the ticket + current host identity and
/// shouldn't be frozen in the event log.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookmarkShared {
    Current(BookmarkSharedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct BookmarkSharedV1 {
    pub brain: BrainName,
    pub bookmark: BookmarkName,
    pub ticket_id: TicketId,
    pub shared_by: ActorId,
}

impl BookmarkShared {
    pub fn build_v1() -> BookmarkSharedV1Builder {
        BookmarkSharedV1::builder()
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn bookmark(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.bookmark,
        }
    }

    pub fn ticket_id(&self) -> TicketId {
        match self {
            Self::Current(v) => v.ticket_id,
        }
    }

    pub fn shared_by(&self) -> ActorId {
        match self {
            Self::Current(v) => v.shared_by,
        }
    }
}

/// Distribution — a follow advanced its checkpoint after a collect
/// operation. The checkpoint is the new position after applying the
/// received events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BookmarkCollected {
    Current(BookmarkCollectedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct BookmarkCollectedV1 {
    pub follow_id: FollowId,
    pub checkpoint: Checkpoint,
    pub events_received: u64,
}

impl BookmarkCollected {
    pub fn build_v1() -> BookmarkCollectedV1Builder {
        BookmarkCollectedV1::builder()
    }

    pub fn follow_id(&self) -> FollowId {
        match self {
            Self::Current(v) => v.follow_id,
        }
    }

    pub fn checkpoint(&self) -> &Checkpoint {
        match self {
            Self::Current(v) => &v.checkpoint,
        }
    }

    pub fn events_received(&self) -> u64 {
        match self {
            Self::Current(v) => v.events_received,
        }
    }
}

/// Distribution — a follow was removed. Only the remote binding is
/// severed; events already collected into the local bookmark stay.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum BookmarkUnfollowed {
    Current(BookmarkUnfollowedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BookmarkUnfollowedV1 {
    pub follow_id: FollowId,
    pub brain: BrainName,
    pub bookmark: BookmarkName,
}

impl BookmarkUnfollowed {
    pub fn build_v1() -> BookmarkUnfollowedV1Builder {
        BookmarkUnfollowedV1::builder()
    }

    pub fn follow_id(&self) -> FollowId {
        match self {
            Self::Current(v) => v.follow_id,
        }
    }

    pub fn brain(&self) -> &BrainName {
        match self {
            Self::Current(v) => &v.brain,
        }
    }

    pub fn bookmark(&self) -> &BookmarkName {
        match self {
            Self::Current(v) => &v.bookmark,
        }
    }
}
