use crate::*;

pub enum PressureDocs {
    List,
    Get,
}

impl PressureDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("pressure")
            .description("Monitor cognitive pressure levels")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-pressure")
                .summary("List pressure readings")
                .description("See all current cognitive pressure measurements across the brain.")
                .build(),
            Self::Get => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-pressure")
                .summary("Get pressure")
                .description(
                    "Retrieve the current cognitive pressure level for a specific context.",
                )
                .build(),
        }
    }
}
