use askama::Template;
use oneiros_model::DreamContext;

#[derive(Template)]
#[template(path = "guidebook.md")]
pub struct GuidebookTemplate<'a> {
    pub context: &'a DreamContext,
}

impl<'a> GuidebookTemplate<'a> {
    pub fn new(context: &'a DreamContext) -> Self {
        Self { context }
    }
}
