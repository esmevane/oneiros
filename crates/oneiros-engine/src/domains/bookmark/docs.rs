use crate::*;

pub(crate) enum BookmarkDocs {
    Create,
    List,
    Switch,
    Merge,
    Share,
    Follow,
    Unfollow,
    Collect,
    Submit,
}

impl BookmarkDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("bookmarks")
            .description("Manage timeline bookmarks")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-bookmark")
                .summary("Create a bookmark")
                .description("Create a new bookmark that defines a named view of the event timeline.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-bookmarks")
                .summary("List bookmarks")
                .description("List all bookmarks known to the current project.")
                .build(),
            Self::Switch => ResourceDocs::builder()
                .tag(tag)
                .nickname("switch-bookmark")
                .summary("Switch to a bookmark")
                .description("Set the active bookmark, making its timeline view the current working context.")
                .build(),
            Self::Merge => ResourceDocs::builder()
                .tag(tag)
                .nickname("merge-bookmark")
                .summary("Merge a bookmark")
                .description("Integrate the events from a bookmark into the current active timeline.")
                .build(),
            Self::Share => ResourceDocs::builder()
                .tag(tag)
                .nickname("share-bookmark")
                .summary("Share a bookmark")
                .description("Produce a shareable `oneiros://` link representing this bookmark's view, optionally scoped by texture.")
                .build(),
            Self::Follow => ResourceDocs::builder()
                .tag(tag)
                .nickname("follow-bookmark")
                .summary("Follow a bookmark link")
                .description("Create a local bookmark by following a remote `oneiros://` link, preserving provenance.")
                .build(),
            Self::Unfollow => ResourceDocs::builder()
                .tag(tag)
                .nickname("unfollow-bookmark")
                .summary("Unfollow a bookmark")
                .description("Remove a followed bookmark, stopping incremental collection from its source.")
                .build(),
            Self::Collect => ResourceDocs::builder()
                .tag(tag)
                .nickname("collect-bookmark")
                .summary("Collect events into a bookmark")
                .description("Collect events from a followed source or directly from a peer host (via --from). Uses the chronicle Merkle diff for efficient sync.")
                .build(),
            Self::Submit => ResourceDocs::builder()
                .tag(tag)
                .nickname("submit-bookmark")
                .summary("Submit a bookmark to a remote")
                .description("Submit a bookmark to a peer host. Requires a write-scoped ticket on the remote. Use --as to rename.")
                .build(),
        }
    }
}
