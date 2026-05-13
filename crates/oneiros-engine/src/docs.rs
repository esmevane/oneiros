use crate::*;

pub(crate) struct AppDocs;

impl AppDocs {
    pub(crate) fn title(&self) -> Label {
        Label::new("Oneiros")
    }

    pub(crate) fn version(&self) -> Label {
        Label::new(env!("CARGO_PKG_VERSION"))
    }

    pub(crate) fn description(&self) -> Description {
        Description::new("Continuous cognition for AI agents")
    }

    pub(crate) fn security_scheme_name(&self) -> Label {
        Label::new("BearerToken")
    }

    pub(crate) fn security_scheme_description(&self) -> Description {
        Description::new("A project access ticket issued via `oneiros ticket issue`")
    }

    pub(crate) fn tags(&self) -> Vec<Tag> {
        vec![
            ActorDocs::Create.tag(),
            AgentDocs::Create.tag(),
            BookmarkDocs::Create.tag(),
            ProjectDocs::Create.tag(),
            CognitionDocs::Add.tag(),
            ConnectionDocs::Create.tag(),
            ContinuityDocs::Emerge.tag(),
            ExperienceDocs::Create.tag(),
            FollowDocs::List.tag(),
            LevelDocs::List.tag(),
            MemoryDocs::Add.tag(),
            NatureDocs::List.tag(),
            PeerDocs::Add.tag(),
            PersonaDocs::List.tag(),
            PressureDocs::List.tag(),
            ProjectDocs::Create.tag(),
            SearchDocs::Search.tag(),
            SeedDocs::SeedCore.tag(),
            SensationDocs::List.tag(),
            StorageDocs::Upload.tag(),
            HostDocs::Init.tag(),
            TenantDocs::Create.tag(),
            TextureDocs::List.tag(),
            TicketDocs::Create.tag(),
            UrgeDocs::List.tag(),
        ]
    }
}
