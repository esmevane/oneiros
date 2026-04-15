use crate::*;

pub struct UrgeView {
    response: UrgeResponse,
}

impl UrgeView {
    pub fn new(response: UrgeResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<UrgeResponse> {
        match self.response {
            UrgeResponse::UrgeSet(name) => {
                let prompt = Confirmation::new("Urge", name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("urge".to_string()).build(),
                );
                Rendered::new(UrgeResponse::UrgeSet(name), prompt, String::new()).with_hints(hints)
            }
            UrgeResponse::UrgeDetails(urge) => {
                let prompt = Detail::new(urge.name.to_string())
                    .field("description:", urge.description.to_string())
                    .field("prompt:", urge.prompt.to_string())
                    .to_string();
                Rendered::new(UrgeResponse::UrgeDetails(urge), prompt, String::new())
            }
            UrgeResponse::Urges(listed) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("description", "Description").max(60),
                ]);
                for urge in &listed.items {
                    table.push_row(vec![urge.name.to_string(), urge.description.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(UrgeResponse::Urges(listed), prompt, String::new())
            }
            UrgeResponse::NoUrges => Rendered::new(
                UrgeResponse::NoUrges,
                format!("{}", "No urges configured.".muted()),
                String::new(),
            ),
            UrgeResponse::UrgeRemoved(name) => {
                let prompt = Confirmation::new("Urge", name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("urge".to_string()).build(),
                );
                Rendered::new(UrgeResponse::UrgeRemoved(name), prompt, String::new())
                    .with_hints(hints)
            }
        }
    }
}
