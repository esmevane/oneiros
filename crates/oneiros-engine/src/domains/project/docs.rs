use crate::*;

pub enum ProjectDocs {
    Init,
    Summary,
}

impl ProjectDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("project")
            .description("Project initialization and overview")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Init => ResourceDocs::builder()
                .tag(tag)
                .nickname("init-project")
                .summary("Initialize project")
                .description("Set up the brain's project database and default projections.")
                .build(),
            Self::Summary => ResourceDocs::builder()
                .tag(tag)
                .nickname("project-summary")
                .summary("Project summary")
                .description("Retrieve an overview of the current brain's project state.")
                .build(),
        }
    }
}
