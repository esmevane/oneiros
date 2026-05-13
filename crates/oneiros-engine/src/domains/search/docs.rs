use crate::*;

pub(crate) enum SearchDocs {
    Search,
}

impl SearchDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("search")
            .description("Search across all entities in a project")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Search => ResourceDocs::builder()
                .tag(tag)
                .nickname("search")
                .summary("Search the project")
                .description(
                    "Query across agents, cognitions, memories, and other entities in the project.",
                )
                .build(),
        }
    }
}
