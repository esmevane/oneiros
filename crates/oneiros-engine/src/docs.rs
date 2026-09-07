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
            ActorRequest::tag(),
            AgentRequest::tag(),
            BookmarkRequest::tag(),
            ProjectRequest::tag(),
            CognitionRequest::tag(),
            ConnectionRequest::tag(),
            ContinuityRequest::tag(),
            ExperienceRequest::tag(),
            FollowRequest::tag(),
            LevelRequest::tag(),
            MemoryRequest::tag(),
            NatureRequest::tag(),
            PeerRequest::tag(),
            PersonaRequest::tag(),
            PressureRequest::tag(),
            ProjectRequest::tag(),
            SearchRequest::tag(),
            SeedRequest::tag(),
            SensationRequest::tag(),
            StorageRequest::tag(),
            HostRequest::tag(),
            LensRequest::tag(),
            TenantRequest::tag(),
            TextureRequest::tag(),
            TicketRequest::tag(),
            TrailRequest::tag(),
            UrgeRequest::tag(),
        ]
    }
}
