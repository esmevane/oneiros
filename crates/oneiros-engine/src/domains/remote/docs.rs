use crate::*;

pub(crate) enum RemoteDocs {
    Add,
    List,
    Remove,
    Bookmarks,
}

impl RemoteDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("remotes")
            .description("Manage remote hosts for bookmark distribution")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Add => ResourceDocs::builder()
                .tag(tag)
                .nickname("add-remote")
                .summary("Add a remote host")
                .description("Register a remote host by providing its project-scoped ticket URI.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-remotes")
                .summary("List remotes")
                .description("List all registered remote hosts.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-remote")
                .summary("Remove a remote")
                .description("Remove a previously registered remote host.")
                .build(),
            Self::Bookmarks => ResourceDocs::builder()
                .tag(tag)
                .nickname("remote-bookmarks")
                .summary("List remote bookmarks")
                .description("List bookmarks available on a remote host.")
                .build(),
        }
    }
}
