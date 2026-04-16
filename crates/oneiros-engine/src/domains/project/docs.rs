use crate::*;

pub enum ProjectDocs {
    Init,
    Summary,
    Activity,
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
            Self::Activity => ResourceDocs::builder()
                .tag(tag)
                .nickname("project-activity")
                .summary("Project activity")
                .description("Stream recent event activity for the current brain project.")
                .build(),
        }
    }
}
