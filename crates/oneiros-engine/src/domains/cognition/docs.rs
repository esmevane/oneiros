use crate::*;

pub enum CognitionDocs {
    Add,
    List,
    Show,
}

impl CognitionDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("cognition")
            .description("Record and review thoughts")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Add => ResourceDocs::builder()
                .tag(tag)
                .nickname("add-cognition")
                .summary("Add a thought")
                .description("Record a new thought for the agent, tagged with a texture that describes its nature.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-cognitions")
                .summary("List thoughts")
                .description("List all thoughts recorded by the agent, optionally filtered by texture.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-cognition")
                .summary("Get a thought")
                .description("Retrieve the full content of a specific thought by ID.")
                .build(),
        }
    }
}
