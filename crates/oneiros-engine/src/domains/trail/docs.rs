use crate::*;

pub(crate) enum TrailDocs {
    Of,
    From,
}

impl TrailDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("trail")
            .description(
                "Walk the events ↔ entities bridge — `of <ref>` lists the events that touched an entity; `from <event-id>` lists the entities an event emitted",
            )
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Of => ResourceDocs::builder()
                .tag(tag)
                .nickname("trail-of")
                .summary("Events that touched an entity")
                .description("Return the events that touched the entity at this ref, oldest first.")
                .build(),
            Self::From => ResourceDocs::builder()
                .tag(tag)
                .nickname("trail-from")
                .summary("Entities emitted by an event")
                .description("Return the entity refs that this event emitted.")
                .build(),
        }
    }
}
