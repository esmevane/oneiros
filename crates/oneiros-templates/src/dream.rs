use askama::Template;
use oneiros_model::{DreamContext, PressureReading, RelevantPressures};

#[derive(Template)]
#[template(path = "dream.md")]
pub struct DreamTemplate<'a> {
    pub context: &'a DreamContext,
    pub pressures: RelevantPressures,
    pub readings: &'a [PressureReading],
}

impl<'a> DreamTemplate<'a> {
    pub fn new(context: &'a DreamContext) -> Self {
        let pressures = RelevantPressures::from_pressures(
            context
                .pressures
                .iter()
                .map(|r| r.pressure.clone())
                .collect(),
        );
        Self {
            context,
            pressures,
            readings: &context.pressures,
        }
    }
}
