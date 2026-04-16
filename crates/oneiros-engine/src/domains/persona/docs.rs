use crate::*;

pub enum PersonaDocs {
    List,
    Set,
    Show,
    Remove,
}

impl PersonaDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("personas")
            .description("Define categories of agents")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-personas")
                .summary("List personas")
                .description("See all defined agent categories.")
                .build(),
            Self::Set => ResourceDocs::builder()
                .tag(tag)
                .nickname("set-persona")
                .summary("Define a persona")
                .description("Create or update an agent category.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-persona")
                .summary("Show a persona")
                .description("Retrieve a single agent category by name.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-persona")
                .summary("Remove a persona")
                .description("Delete an agent category from the brain.")
                .build(),
        }
    }
}
