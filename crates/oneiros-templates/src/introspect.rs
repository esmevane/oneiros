use askama::Template;
use oneiros_model::Agent;

#[derive(Template)]
#[template(path = "introspect.md")]
pub struct IntrospectTemplate<'a> {
    pub agent: &'a Agent,
}

impl<'a> IntrospectTemplate<'a> {
    pub fn new(agent: &'a Agent) -> Self {
        Self { agent }
    }
}
