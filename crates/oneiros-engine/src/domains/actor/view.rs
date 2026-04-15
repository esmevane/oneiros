use crate::*;

pub struct ActorView {
    response: ActorResponse,
}

impl ActorView {
    pub fn new(response: ActorResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<ActorResponse> {
        match self.response {
            ActorResponse::Created(wrapped) => {
                let prompt = Confirmation::new("Actor", wrapped.data.name.to_string(), "created")
                    .to_string();
                Rendered::new(ActorResponse::Created(wrapped), prompt, String::new())
            }
            ActorResponse::Found(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("id:", wrapped.data.id.to_string())
                    .field("tenant_id:", wrapped.data.tenant_id.to_string())
                    .to_string();
                Rendered::new(ActorResponse::Found(wrapped), prompt, String::new())
            }
            ActorResponse::Listed(listed) => {
                let mut table =
                    Table::new(vec![Column::key("name", "Name"), Column::key("id", "ID")]);
                for wrapped in &listed.items {
                    let actor = &wrapped.data;
                    table.push_row(vec![actor.name.to_string(), actor.id.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(ActorResponse::Listed(listed), prompt, String::new())
            }
        }
    }
}
