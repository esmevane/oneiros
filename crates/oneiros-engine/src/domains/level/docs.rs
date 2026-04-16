use crate::*;

pub enum LevelDocs {
    List,
    Set,
    Show,
    Remove,
}

impl LevelDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("levels")
            .description("Define memory retention tiers")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-levels")
                .summary("List levels")
                .description("List all memory retention levels defined for the current brain.")
                .build(),
            Self::Set => ResourceDocs::builder()
                .tag(tag)
                .nickname("set-level")
                .summary("Set a level")
                .description("Define or update a named memory retention tier with its priority and eviction policy.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-level")
                .summary("Get a level")
                .description("Look up the configuration of a specific memory retention level by name.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-level")
                .summary("Remove a level")
                .description("Delete a memory retention level, preventing new memories from being classified under it.")
                .build(),
        }
    }
}
