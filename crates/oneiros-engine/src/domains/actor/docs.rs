use crate::*;

pub enum ActorDocs {
    Create,
    List,
    Show,
}

impl ActorDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("actors")
            .description("Manage actors within a tenant")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-actor")
                .summary("Create an actor")
                .description("Register a new actor under the current tenant.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-actors")
                .summary("List actors")
                .description("List all actors visible to the current tenant.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-actor")
                .summary("Get an actor")
                .description("Look up a specific actor by ID within the current tenant.")
                .build(),
        }
    }
}
