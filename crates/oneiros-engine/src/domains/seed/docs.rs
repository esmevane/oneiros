use crate::*;

pub(crate) enum SeedDocs {
    SeedCore,
    SeedAgents,
}

impl SeedDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("seed")
            .description("Plant initial vocabulary and agents")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::SeedCore => ResourceDocs::builder()
                .tag(tag)
                .nickname("seed-core")
                .summary("Seed core vocabulary")
                .description("Plant the foundational textures, sensations, urges, and personas into the project.")
                .build(),
            Self::SeedAgents => ResourceDocs::builder()
                .tag(tag)
                .nickname("seed-agents")
                .summary("Seed agents")
                .description("Plant a default set of agents into the project to bootstrap cognition.")
                .build(),
        }
    }
}
