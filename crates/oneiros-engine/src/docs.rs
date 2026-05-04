use crate::*;

pub struct AppDocs;

impl AppDocs {
    pub fn title(&self) -> Label {
        Label::new("Oneiros")
    }

    pub fn version(&self) -> Label {
        Label::new(env!("CARGO_PKG_VERSION"))
    }

    pub fn description(&self) -> Description {
        Description::new("Continuous cognition for AI agents")
    }

    pub fn security_scheme_name(&self) -> Label {
        Label::new("BearerToken")
    }

    pub fn security_scheme_description(&self) -> Description {
        Description::new("A brain access ticket issued via `oneiros ticket issue`")
    }

    pub fn tags(&self) -> Vec<Tag> {
        vec![
            ActorDocs::Create.tag(),
            AgentDocs::Create.tag(),
            BookmarkDocs::Create.tag(),
            BrainDocs::Create.tag(),
            CognitionDocs::Add.tag(),
            ConnectionDocs::Create.tag(),
            ContinuityDocs::Emerge.tag(),
            ExperienceDocs::Create.tag(),
            LevelDocs::List.tag(),
            MemoryDocs::Add.tag(),
            NatureDocs::List.tag(),
            PeerDocs::Add.tag(),
            PersonaDocs::List.tag(),
            PressureDocs::List.tag(),
            ProjectDocs::Init.tag(),
            SearchDocs::Search.tag(),
            SeedDocs::SeedCore.tag(),
            SensationDocs::List.tag(),
            StorageDocs::Upload.tag(),
            SystemDocs::Init.tag(),
            TenantDocs::Create.tag(),
            TextureDocs::List.tag(),
            TicketDocs::Create.tag(),
            UrgeDocs::List.tag(),
        ]
    }
}
