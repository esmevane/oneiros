use crate::*;

pub enum ConnectionDocs {
    Create,
    List,
    Show,
    Remove,
}

impl ConnectionDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("connections")
            .description("Draw and manage relationships between entities")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-connection")
                .summary("Create a connection")
                .description("Draw a typed relationship between two entities using a defined nature.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-connections")
                .summary("List connections")
                .description("List all relationships visible to the current brain, optionally filtered by nature or entity.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-connection")
                .summary("Get a connection")
                .description("Look up the details of a specific relationship by ID.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-connection")
                .summary("Remove a connection")
                .description("Delete a relationship between entities, removing it from the graph.")
                .build(),
        }
    }
}
