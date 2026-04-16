use crate::*;

pub enum TicketDocs {
    Create,
    List,
    Show,
    Validate,
}

impl TicketDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("tickets")
            .description("Issue and manage brain access tickets")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-ticket")
                .summary("Issue a ticket")
                .description("Generate a new access ticket granting entry to a brain.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-tickets")
                .summary("List tickets")
                .description("See all access tickets issued for this brain.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-ticket")
                .summary("Show a ticket")
                .description("Retrieve the details of a specific brain access ticket.")
                .build(),
            Self::Validate => ResourceDocs::builder()
                .tag(tag)
                .nickname("validate-ticket")
                .summary("Validate a ticket")
                .description("Check whether a brain access ticket is currently valid.")
                .build(),
        }
    }
}
