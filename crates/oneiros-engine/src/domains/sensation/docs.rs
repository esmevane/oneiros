use crate::*;

pub enum SensationDocs {
    List,
    Set,
    Show,
    Remove,
}

impl SensationDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("sensations")
            .description("Define qualities of experience")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-sensations")
                .summary("List sensations")
                .description("See all defined qualities of experience available to agents.")
                .build(),
            Self::Set => ResourceDocs::builder()
                .tag(tag)
                .nickname("set-sensation")
                .summary("Define a sensation")
                .description("Create or update a quality of experience in the brain's vocabulary.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-sensation")
                .summary("Show a sensation")
                .description("Retrieve a single quality of experience by name.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-sensation")
                .summary("Remove a sensation")
                .description("Delete a quality of experience from the brain's vocabulary.")
                .build(),
        }
    }
}
