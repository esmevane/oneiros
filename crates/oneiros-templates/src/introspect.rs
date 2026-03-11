use askama::Template;
use oneiros_model::{Agent, RelevantPressures};

#[derive(Template)]
#[template(path = "introspect.md")]
pub struct IntrospectTemplate<'a> {
    pub agent: &'a Agent,
    pub pressures: RelevantPressures,
}

impl<'a> IntrospectTemplate<'a> {
    pub fn new(agent: &'a Agent, pressures: RelevantPressures) -> Self {
        Self { agent, pressures }
    }
}
