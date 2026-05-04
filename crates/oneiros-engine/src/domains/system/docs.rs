use crate::*;

pub enum SystemDocs {
    Init,
}

impl SystemDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("system")
            .description("Host-level initialization and bootstrap")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Init => ResourceDocs::builder()
                .tag(tag)
                .nickname("init-system")
                .summary("Initialize host")
                .description(
                    "Create the host data directory, generate the host keypair, and seed the default tenant and actor. Refuses once a tenant exists.",
                )
                .build(),
        }
    }
}
