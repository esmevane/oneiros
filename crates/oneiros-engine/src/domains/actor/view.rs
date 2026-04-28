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
            ActorResponse::Created(ActorCreatedResponse::V1(created)) => {
                let prompt = Confirmation::new("Actor", created.actor.name.to_string(), "created")
                    .to_string();
                Rendered::new(
                    ActorResponse::Created(ActorCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            ActorResponse::Found(ActorFoundResponse::V1(found)) => {
                let prompt = Detail::new(found.actor.name.to_string())
                    .field("id:", found.actor.id.to_string())
                    .field("tenant_id:", found.actor.tenant_id.to_string())
                    .to_string();
                Rendered::new(
                    ActorResponse::Found(ActorFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            ActorResponse::Listed(ActorsResponse::V1(listed)) => {
                let mut table =
                    Table::new(vec![Column::key("name", "Name"), Column::key("id", "ID")]);
                for actor in &listed.items {
                    table.push_row(vec![actor.name.to_string(), actor.id.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    ActorResponse::Listed(ActorsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
