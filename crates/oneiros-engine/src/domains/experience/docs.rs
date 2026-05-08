use crate::*;

pub(crate) enum ExperienceDocs {
    Create,
    List,
    Show,
}

impl ExperienceDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("experiences")
            .description("Mark and revisit meaningful moments")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-experience")
                .summary("Create an experience")
                .description("Mark a meaningful moment in the agent's timeline with a description and sensation.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-experiences")
                .summary("List experiences")
                .description("List all marked experiences in the agent's history, optionally filtered by sensation.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-experience")
                .summary("Get an experience")
                .description("Retrieve the full record of a specific marked moment by ID.")
                .build(),
        }
    }
}
