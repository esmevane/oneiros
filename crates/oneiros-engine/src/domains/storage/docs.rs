use crate::*;

pub enum StorageDocs {
    Upload,
    List,
    Show,
    Remove,
}

impl StorageDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("storage")
            .description("Archive and retrieve files")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Upload => ResourceDocs::builder()
                .tag(tag)
                .nickname("upload-blob")
                .summary("Upload a file")
                .description("Store a file as a blob in the brain's archive.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-blobs")
                .summary("List files")
                .description("See all blobs currently stored in the brain's archive.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-blob")
                .summary("Show a file")
                .description("Retrieve metadata and content for a specific archived blob.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-blob")
                .summary("Remove a file")
                .description("Delete a blob from the brain's archive.")
                .build(),
        }
    }
}
