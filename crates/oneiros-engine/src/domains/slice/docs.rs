use crate::*;

pub(crate) enum SliceDocs {
    Create,
}

impl SliceDocs {
    pub(crate) fn tag(&self) -> Tag {
        Tag::builder()
            .name("slices")
            .description("Standing lens-filtered views over continuity")
            .build()
    }

    pub(crate) fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Create => ResourceDocs::builder()
                .tag(tag)
                .nickname("create-slice")
                .summary("Create a slice")
                .description(
                    "Create a standing lens-filtered view of the event stream, \
                     materializing matching events retroactively.",
                )
                .build(),
        }
    }
}
