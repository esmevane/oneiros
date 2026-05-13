use crate::*;

pub(crate) enum AgentDocs {
    Create,
    List,
    Show,
    Update,
    Remove,
}

impl AgentDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("agents")
            .description("Manage cognitive agents")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-agent")
                .summary("Create an agent")
                .description("Register a new cognitive agent under the current project.")
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-agents")
                .summary("List agents")
                .description("List all cognitive agents registered in the current project.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-agent")
                .summary("Get an agent")
                .description("Look up a specific cognitive agent by name or ID.")
                .build(),
            Self::Update => ResourceDocs::builder()
                .tag(tag)
                .nickname("update-agent")
                .summary("Update an agent")
                .description("Modify the configuration or metadata of an existing cognitive agent.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-agent")
                .summary("Remove an agent")
                .description("Permanently remove a cognitive agent and all associated records from the project.")
                .build(),
        }
    }
}
