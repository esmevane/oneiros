use crate::*;

pub enum UrgeDocs {
    List,
    Set,
    Show,
    Remove,
}

impl UrgeDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("urges")
            .description("Define cognitive drives")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-urges")
                .summary("List urges")
                .description("See all defined cognitive drives available to agents.")
                .build(),
            Self::Set => ResourceDocs::builder()
                .tag(tag)
                .nickname("set-urge")
                .summary("Define an urge")
                .description("Create or update a cognitive drive in the brain's vocabulary.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-urge")
                .summary("Show an urge")
                .description("Retrieve a single cognitive drive by name.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-urge")
                .summary("Remove an urge")
                .description("Delete a cognitive drive from the brain's vocabulary.")
                .build(),
        }
    }
}
