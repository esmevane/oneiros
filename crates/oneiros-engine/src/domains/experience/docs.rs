use crate::*;

pub enum ExperienceDocs {
    Create,
    List,
    Show,
    UpdateDescription,
    UpdateSensation,
}

impl ExperienceDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("experiences")
            .description("Mark and revisit meaningful moments")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-experience")
                .summary("Create an experience")
                .description("Mark a meaningful moment in the agent's timeline with a description and sensation.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-experiences")
                .summary("List experiences")
                .description("List all marked experiences in the agent's history, optionally filtered by sensation.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-experience")
                .summary("Get an experience")
                .description("Retrieve the full record of a specific marked moment by ID.")
                .build(),
            Self::UpdateDescription => ResourceDocs::builder()
                .tag(tag)
                .nickname("update-experience-description")
                .summary("Update experience description")
                .description("Revise the descriptive text of an existing experience to better capture its meaning.")
                .build(),
            Self::UpdateSensation => ResourceDocs::builder()
                .tag(tag)
                .nickname("update-experience-sensation")
                .summary("Update experience sensation")
                .description("Change the sensation label of an existing experience to reclassify its affective quality.")
                .build(),
        }
    }
}
