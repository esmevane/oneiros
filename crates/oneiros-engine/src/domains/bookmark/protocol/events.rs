use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(
    kind = BookmarkEventsType,
    display = "kebab-case",
    attrs(
        expect(
            clippy::enum_variant_names,
            reason = "We use these for `type` notation in serde"
        )
    )
)]
#[expect(
    clippy::enum_variant_names,
    reason = "We use these for `type` notation in serde"
)]
pub(crate) enum BookmarkEvents {
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
    pub(crate) fn maybe_bookmark(&self) -> Option<Bookmark> {
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
    pub(crate) enum BookmarkCreated {
        V1 => {
            #[serde(flatten)] pub(crate) bookmark: Bookmark,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkForked {
        V1 => {
            #[serde(flatten)] pub(crate) bookmark: Bookmark,
            pub(crate) from: BookmarkName,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkSwitched {
        V1 => {
            #[serde(alias = "brain")] pub(crate) project: ProjectName,
            pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkMerged {
        V1 => {
            #[serde(alias = "brain")] pub(crate) project: ProjectName,
            pub(crate) source: BookmarkName,
            pub(crate) target: BookmarkName,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkShared {
        V1 => {
            #[serde(alias = "brain")] pub(crate) project: ProjectName,
            pub(crate) bookmark: BookmarkName,
            pub(crate) ticket_id: TicketId,
            pub(crate) shared_by: ActorId,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkFollowed {
        V1 => {
            #[builder(default)] pub(crate) id: FollowId,
            #[serde(alias = "brain")] pub(crate) project: ProjectName,
            pub(crate) bookmark: BookmarkName,
            pub(crate) source: FollowSource,
            #[builder(default = Checkpoint::empty())] pub(crate) checkpoint: Checkpoint,
            #[builder(default = Timestamp::now())] pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkCollected {
        V1 => {
            pub(crate) follow_id: FollowId,
            pub(crate) checkpoint: Checkpoint,
            pub(crate) events_received: u64,
        }
    }
}

versioned! {
    pub(crate) enum BookmarkUnfollowed {
        V1 => {
            pub(crate) follow_id: FollowId,
            #[serde(alias = "brain")] pub(crate) project: ProjectName,
            pub(crate) bookmark: BookmarkName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bookmark() -> Bookmark {
        Bookmark::builder()
            .project(ProjectName::new("test-project"))
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
        assert_eq!(json["data"]["project"], "test-project");
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

    #[test]
    fn legacy_bookmark_created_with_brain_field_decodes_via_v1() {
        let bookmark_id = BookmarkId::new();
        let json = serde_json::json!({
            "type": "bookmark-created",
            "data": {
                "id": bookmark_id.to_string(),
                "brain": "legacy-project",
                "name": "main",
                "created_at": "2026-01-01T00:00:00Z"
            }
        });
        let event: BookmarkEvents = serde_json::from_value(json).expect("legacy decode");
        let created = match event {
            BookmarkEvents::BookmarkCreated(inner) => inner.current().unwrap(),
            other => panic!("expected BookmarkCreated, got {other:?}"),
        };
        assert_eq!(created.bookmark.project.as_str(), "legacy-project");
        assert_eq!(created.bookmark.name.as_str(), "main");
    }

    #[test]
    fn legacy_bookmark_switched_with_brain_field_decodes_via_v1() {
        let json = serde_json::json!({
            "type": "bookmark-switched",
            "data": {
                "brain": "legacy-project",
                "name": "main"
            }
        });
        let event: BookmarkEvents = serde_json::from_value(json).expect("legacy decode");
        let switched = match event {
            BookmarkEvents::BookmarkSwitched(inner) => inner.current().unwrap(),
            other => panic!("expected BookmarkSwitched, got {other:?}"),
        };
        assert_eq!(switched.project.as_str(), "legacy-project");
        assert_eq!(switched.name.as_str(), "main");
    }
}
