use crate::*;

pub(crate) enum ActorDocs {
    Create,
    Get,
    List,
}

impl ActorDocs {
    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = ActorOperations::tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-actor")
                .summary(CreateActor::SUMMARY)
                .description(CreateActor::DESCRIPTION)
                .content(CreateActor::content())
                .build(),
            Self::Get => ResourceDocs::builder()
                .tag(tag)
                .nickname("get-actor")
                .summary(GetActor::SUMMARY)
                .description(GetActor::DESCRIPTION)
                .content(GetActor::content())
                .build(),
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-actors")
                .summary(ListActors::SUMMARY)
                .description(ListActors::DESCRIPTION)
                .content(ListActors::content())
                .build(),
        }
    }
}
