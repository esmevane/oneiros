use crate::*;

pub(crate) enum HostDocs {
    Init,
}

impl HostDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("host")
            .description("Host-level initialization and bootstrap")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Init => ResourceDocs::builder()
                .tag(tag)
                .nickname("init-host")
                .summary("Initialize host")
                .description(
                    "Create the host data directory, generate the host keypair, and seed the default tenant and actor. Refuses once a tenant exists.",
                )
                .build(),
        }
    }
}
