use crate::*;

pub(crate) enum FollowDocs {
    Get,
    List,
}

impl FollowDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("follows")
            .description(
                "Inspect follow records — links between local bookmarks and the sources they track",
            )
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Get => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-follow")
                .summary("Get a follow")
                .description("Retrieve a single follow record by its identifier.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-follows")
                .summary("List follows")
                .description("List all follow records on this host.")
                .build(),
        }
    }
}
