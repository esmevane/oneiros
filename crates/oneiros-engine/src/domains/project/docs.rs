use crate::*;

pub(crate) enum ProjectDocs {
    Create,
    List,
    Show,
    Share,
    Follow,
}

impl ProjectDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("projects")
            .description("Manage projects on this host")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-project")
                .summary("Create a project")
                .description(
                    "Provision a new project on this host: insert it into the host index, \
                     open its event log and default bookmark, and issue an access token.",
                )
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-projects")
                .summary("List projects")
                .description("List all projects provisioned on this host.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-project")
                .summary("Get a project")
                .description("Look up details for a specific project by name or ID.")
                .build(),
            Self::Share => ResourceDocs::builder()
                .tag(tag)
                .nickname("share-project")
                .summary("Share a project")
                .description("Issue a project-scoped ticket and print the URI for peer access.")
                .build(),
            Self::Follow => ResourceDocs::builder()
                .tag(tag)
                .nickname("follow-project")
                .summary("Follow a project")
                .description("Create a repository peer by following a project share URI.")
                .build(),
        }
    }
}
