use askama::Template;
use oneiros_model::DreamContext;

#[derive(Template)]
#[template(path = "dream.md")]
pub struct DreamTemplate<'a> {
    pub context: &'a DreamContext,
}

impl<'a> DreamTemplate<'a> {
    pub fn new(context: &'a DreamContext) -> Self {
        Self { context }
    }
}
