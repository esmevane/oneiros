use crate::*;

pub enum MemoryDocs {
    Add,
    List,
    Show,
}

impl MemoryDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("memory")
            .description("Consolidate and review knowledge")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Add => ResourceDocs::builder()
                .tag(tag)
                .nickname("add-memory")
                .summary("Add a memory")
                .description("Store a piece of consolidated knowledge for the agent at a specified retention level.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-memories")
                .summary("List memories")
                .description("List all memories held by the agent, optionally filtered by retention level.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-memory")
                .summary("Get a memory")
                .description("Retrieve the full content of a specific memory by ID.")
                .build(),
        }
    }
}
