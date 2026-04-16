use crate::*;

pub enum TenantDocs {
    Create,
    List,
    Show,
}

impl TenantDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("tenants")
            .description("Manage tenants on this host")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-tenant")
                .summary("Create a tenant")
                .description("Register a new tenant brain on this host.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-tenants")
                .summary("List tenants")
                .description("See all tenant brains registered on this host.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-tenant")
                .summary("Show a tenant")
                .description("Retrieve details for a specific tenant brain on this host.")
                .build(),
        }
    }
}
