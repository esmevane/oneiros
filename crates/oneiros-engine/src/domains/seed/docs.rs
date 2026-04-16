use crate::*;

pub enum SeedDocs {
    SeedCore,
    SeedAgents,
}

impl SeedDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("seed")
            .description("Plant initial vocabulary and agents")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::SeedCore => ResourceDocs::builder()
                .tag(tag)
                .nickname("seed-core")
                .summary("Seed core vocabulary")
                .description("Plant the foundational textures, sensations, urges, and personas into the brain.")
                .build(),
            Self::SeedAgents => ResourceDocs::builder()
                .tag(tag)
                .nickname("seed-agents")
                .summary("Seed agents")
                .description("Plant a default set of agents into the brain to bootstrap cognition.")
                .build(),
        }
    }
}
