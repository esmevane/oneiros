use crate::*;

pub enum PeerDocs {
    Add,
    List,
    Show,
    Remove,
}

impl PeerDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("peers")
            .description("Manage peer connections for distribution")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Add => ResourceDocs::builder()
                .tag(tag)
                .nickname("add-peer")
                .summary("Add a peer")
                .description(
                    "Register a remote brain as a peer, enabling event exchange and distribution.",
                )
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-peers")
                .summary("List peers")
                .description("List all remote brains registered as peers of the current brain.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-peer")
                .summary("Get a peer")
                .description("Look up the connection details for a specific registered peer.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-peer")
                .summary("Remove a peer")
                .description(
                    "Deregister a remote brain as a peer, stopping further event exchange with it.",
                )
                .build(),
        }
    }
}
