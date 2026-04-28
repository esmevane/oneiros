use crate::*;

pub struct BrainView {
    response: BrainResponse,
}

impl BrainView {
    pub fn new(response: BrainResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<BrainResponse> {
        match self.response {
            BrainResponse::Created(BrainCreatedResponse::V1(created)) => {
                let prompt = Confirmation::new("Brain", created.brain.name.to_string(), "created")
                    .to_string();
                Rendered::new(
                    BrainResponse::Created(BrainCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            BrainResponse::Found(BrainFoundResponse::V1(found)) => {
                let prompt = Detail::new(found.brain.name.to_string()).to_string();
                Rendered::new(
                    BrainResponse::Found(BrainFoundResponse::V1(found)),
                    prompt,
                    String::new(),
                )
            }
            BrainResponse::Listed(BrainsResponse::V1(listed)) => {
                let mut table = Table::new(vec![Column::key("name", "Name")]);
                for brain in &listed.items {
                    table.push_row(vec![brain.name.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    BrainResponse::Listed(BrainsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
