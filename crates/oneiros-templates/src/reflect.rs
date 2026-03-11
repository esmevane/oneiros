use askama::Template;
use oneiros_model::{Agent, RelevantPressures};

#[derive(Template)]
#[template(path = "reflect.md")]
pub struct ReflectTemplate<'a> {
    pub agent: &'a Agent,
    pub pressures: RelevantPressures,
}

impl<'a> ReflectTemplate<'a> {
    pub fn new(agent: &'a Agent, pressures: RelevantPressures) -> Self {
        Self { agent, pressures }
    }
}
