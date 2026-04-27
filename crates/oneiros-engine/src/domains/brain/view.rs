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
            BrainResponse::Created(wrapped) => {
                let prompt = Confirmation::new("Brain", wrapped.data.name().to_string(), "created")
                    .to_string();
                Rendered::new(BrainResponse::Created(wrapped), prompt, String::new())
            }
            BrainResponse::Found(wrapped) => {
                let prompt = Detail::new(wrapped.data.name().to_string()).to_string();
                Rendered::new(BrainResponse::Found(wrapped), prompt, String::new())
            }
            BrainResponse::Listed(listed) => {
                let mut table = Table::new(vec![Column::key("name", "Name")]);
                for wrapped in &listed.items {
                    table.push_row(vec![wrapped.data.name().to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(BrainResponse::Listed(listed), prompt, String::new())
            }
        }
    }
}
