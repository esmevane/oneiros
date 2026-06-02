use crate::*;

pub(crate) enum SliceDocs {
    Create,
    List,
    Delete,
    Diff,
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
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-slices")
                .summary("List slices")
                .description("List all slices for the current project.")
                .build(),
            Self::Delete => ResourceDocs::builder()
                .tag(tag)
                .nickname("delete-slice")
                .summary("Delete a slice")
                .description("Delete a slice by name.")
                .build(),
            Self::Diff => ResourceDocs::builder()
                .tag(tag)
                .nickname("diff-slices")
                .summary("Diff two slices")
                .description("Compare two slices and return the event counts unique to each and shared between them.")
                .build(),
        }
    }
}
