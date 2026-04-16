use crate::*;

pub enum NatureDocs {
    List,
    Set,
    Show,
    Remove,
}

impl NatureDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("natures")
            .description("Define kinds of relationships")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-natures")
                .summary("List natures")
                .description("List all relationship kinds defined for the current brain.")
                .build(),
            Self::Set => ResourceDocs::builder()
                .tag(tag)
                .nickname("set-nature")
                .summary("Set a nature")
                .description("Define or update a named relationship kind that can be used to type connections between entities.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-nature")
                .summary("Get a nature")
                .description("Look up the definition of a specific relationship kind by name.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-nature")
                .summary("Remove a nature")
                .description("Delete a relationship kind, preventing it from being assigned to new connections.")
                .build(),
        }
    }
}
