use crate::*;

pub enum TextureDocs {
    List,
    Set,
    Show,
    Remove,
}

impl TextureDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("textures")
            .description("Define qualities of thought")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::List => ResourceDocs::builder()
                .tag(tag)
                .nickname("list-textures")
                .summary("List textures")
                .description("See all defined qualities of thought available to agents.")
                .build(),
            Self::Set => ResourceDocs::builder()
                .tag(tag)
                .nickname("set-texture")
                .summary("Define a texture")
                .description("Create or update a quality of thought in the brain's vocabulary.")
                .build(),
            Self::Show => ResourceDocs::builder()
                .tag(tag)
                .nickname("show-texture")
                .summary("Show a texture")
                .description("Retrieve a single quality of thought by name.")
                .build(),
            Self::Remove => ResourceDocs::builder()
                .tag(tag)
                .nickname("remove-texture")
                .summary("Remove a texture")
                .description("Delete a quality of thought from the brain's vocabulary.")
                .build(),
        }
    }
}
