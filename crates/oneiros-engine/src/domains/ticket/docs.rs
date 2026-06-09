use crate::*;

pub(crate) enum TicketDocs {
    Create,
    List,
    Show,
    Validate,
}

impl TicketDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("tickets")
            .description("Issue and manage project access tickets")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-ticket")
                .summary("Issue a ticket")
                .description("Generate a new access ticket granting entry to a project.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-tickets")
                .summary("List tickets")
                .description("See all access tickets issued for this project.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-ticket")
                .summary("Show a ticket")
                .description("Retrieve the details of a specific project access ticket.")
                .build(),
            Self::Validate => ResourceDocs::builder()
                .tag(tag)
                .nickname("validate-ticket")
                .summary("Validate a ticket")
                .description("Verify that a ticket is still valid and not revoked.")
                .build(),
        }
    }
}
