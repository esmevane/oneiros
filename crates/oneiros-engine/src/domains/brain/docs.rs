use crate::*;

pub enum BrainDocs {
    Create,
    List,
    Show,
}

impl BrainDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("brains")
            .description("Manage brains on this host")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-brain")
                .summary("Create a brain")
                .description(
                    "Provision a new brain on this host, giving it a unique identity and storage.",
                )
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-brains")
                .summary("List brains")
                .description("List all brains provisioned on this host.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-brain")
                .summary("Get a brain")
                .description("Look up details for a specific brain by name or ID.")
                .build(),
        }
    }
}
