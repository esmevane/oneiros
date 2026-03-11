use askama::Template;
use oneiros_model::{Agent, RelevantPressures};

#[derive(Template)]
#[template(path = "sense.md")]
pub struct SenseTemplate<'a> {
    pub agent: &'a Agent,
    pub event_data: &'a str,
    pub pressures: RelevantPressures,
}

impl<'a> SenseTemplate<'a> {
    pub fn new(agent: &'a Agent, event_data: &'a str, pressures: RelevantPressures) -> Self {
        Self {
            agent,
            event_data,
            pressures,
        }
    }
}
