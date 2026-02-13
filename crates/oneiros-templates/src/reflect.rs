use askama::Template;
use oneiros_model::Agent;

#[derive(Template)]
#[template(path = "reflect.md")]
pub struct ReflectTemplate<'a> {
    pub agent: &'a Agent,
}

impl<'a> ReflectTemplate<'a> {
    pub fn new(agent: &'a Agent) -> Self {
        Self { agent }
    }
}
