use crate::*;

pub(crate) enum PeerDocs {
    Add,
    List,
    Show,
    Remove,
}

impl PeerDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("peers")
            .description("Manage peer connections for distribution")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Add => ResourceDocs::builder()
                .tag(tag)
                .nickname("add-peer")
                .summary("Add a peer")
                .description(
                    "Register a remote host as a peer. Provide an oneiros:// URI to add a remote peer with ticket-based auth, or a plain address for a follow peer.",
                )
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-peers")
                .summary("List peers")
                .description("List all known peer hosts.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-peer")
                .summary("Get a peer")
                .description("Look up the connection details for a specific peer.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-peer")
                .summary("Remove a peer")
                .description("Deregister a peer host.")
                .build(),
        }
    }
}
