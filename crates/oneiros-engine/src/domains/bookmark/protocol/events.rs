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
    /// Distribution — a local bookmark begins following a source.
    BookmarkFollowed(BookmarkFollowed),
    /// Distribution — a follow advanced its checkpoint after a collect.
    BookmarkCollected(BookmarkCollected),
    /// Distribution — a follow was removed.
    BookmarkUnfollowed(BookmarkUnfollowed),
}

impl BookmarkEvents {
    /// The full `Bookmark` carried by creation-style events. `Forked`
    /// also carries one (the new fork) — its `from` is a reference, not
    /// a full record.
    pub fn maybe_bookmark(&self) -> Option<Bookmark> {
        match self {
            BookmarkEvents::BookmarkCreated(event) => {
                event.clone().current().ok().map(|v| v.bookmark)
            }
            BookmarkEvents::BookmarkForked(event) => {
                event.clone().current().ok().map(|v| v.bookmark)
            }
            BookmarkEvents::BookmarkSwitched(_)
            | BookmarkEvents::BookmarkMerged(_)
            | BookmarkEvents::BookmarkShared(_)
            | BookmarkEvents::BookmarkFollowed(_)
            | BookmarkEvents::BookmarkCollected(_)
            | BookmarkEvents::BookmarkUnfollowed(_) => None,
        }
    }
}

versioned! {
    pub enum BookmarkCreated {
        V1 => {
            #[serde(flatten)] pub bookmark: Bookmark,
        }
    }
}

versioned! {
    pub enum BookmarkForked {
        V1 => {
            #[serde(flatten)] pub bookmark: Bookmark,
            pub from: BookmarkName,
        }
    }
}

versioned! {
    pub enum BookmarkSwitched {
        V1 => {
            pub brain: BrainName,
            pub name: BookmarkName,
        }
    }
}

versioned! {
    pub enum BookmarkMerged {
        V1 => {
            pub brain: BrainName,
            pub source: BookmarkName,
            pub target: BookmarkName,
        }
    }
}

versioned! {
    pub enum BookmarkShared {
        V1 => {
            pub brain: BrainName,
            pub bookmark: BookmarkName,
            pub ticket_id: TicketId,
            pub shared_by: ActorId,
        }
    }
}

versioned! {
    pub enum BookmarkFollowed {
        V1 => {
            #[builder(default)] pub id: FollowId,
            pub brain: BrainName,
            pub bookmark: BookmarkName,
            pub source: FollowSource,
            #[builder(default = Checkpoint::empty())] pub checkpoint: Checkpoint,
            #[builder(default = Timestamp::now())] pub created_at: Timestamp,
        }
    }
}

versioned! {
    pub enum BookmarkCollected {
        V1 => {
            pub follow_id: FollowId,
            pub checkpoint: Checkpoint,
            pub events_received: u64,
        }
    }
}

versioned! {
    pub enum BookmarkUnfollowed {
        V1 => {
            pub follow_id: FollowId,
            pub brain: BrainName,
            pub bookmark: BookmarkName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bookmark() -> Bookmark {
        Bookmark::builder()
            .brain(BrainName::new("test-brain"))
            .name(BookmarkName::new("main"))
            .build()
    }

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

    #[test]
    fn bookmark_created_wire_format_is_flat() {
        let bookmark = sample_bookmark();
        let event = BookmarkEvents::BookmarkCreated(BookmarkCreated::V1(BookmarkCreatedV1 {
            bookmark: bookmark.clone(),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "bookmark-created");
        assert!(
            json["data"].get("bookmark").is_none(),
            "flatten must elide the bookmark envelope on the wire"
        );
        assert_eq!(json["data"]["id"], bookmark.id.to_string());
        assert_eq!(json["data"]["name"], "main");
        assert_eq!(json["data"]["brain"], "test-brain");
        assert!(json["data"].get("created_at").is_some());
    }

    #[test]
    fn bookmark_forked_wire_format_is_flat() {
        let bookmark = sample_bookmark();
        let event = BookmarkEvents::BookmarkForked(BookmarkForked::V1(BookmarkForkedV1 {
            bookmark: bookmark.clone(),
            from: BookmarkName::new("main"),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "bookmark-forked");
        assert!(
            json["data"].get("bookmark").is_none(),
            "flatten must elide the bookmark envelope on the wire"
        );
        assert_eq!(json["data"]["id"], bookmark.id.to_string());
        assert_eq!(json["data"]["from"], "main");
    }
}
