use askama::Template;
use oneiros_model::{DreamContext, RelevantPressures};

#[derive(Template)]
#[template(path = "dream.md")]
pub struct DreamTemplate<'a> {
    pub context: &'a DreamContext,
    pub pressures: RelevantPressures,
}

impl<'a> DreamTemplate<'a> {
    pub fn new(context: &'a DreamContext) -> Self {
        let pressures = RelevantPressures::from_readings(context.pressures.clone());
        Self { context, pressures }
    }
}
