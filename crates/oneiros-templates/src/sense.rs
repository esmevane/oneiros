use askama::Template;
use oneiros_model::Agent;

#[derive(Template)]
#[template(path = "sense.md")]
pub struct SenseTemplate<'a> {
    pub agent: &'a Agent,
    pub event_data: &'a str,
}

impl<'a> SenseTemplate<'a> {
    pub fn new(agent: &'a Agent, event_data: &'a str) -> Self {
        Self { agent, event_data }
    }
}
