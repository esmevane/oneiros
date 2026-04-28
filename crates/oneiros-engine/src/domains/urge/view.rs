use crate::*;

pub struct UrgeView {
    response: UrgeResponse,
}

impl UrgeView {
    pub fn new(response: UrgeResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            UrgeResponse::Urges(UrgesResponse::V1(listed)) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|item| (item.name.to_string(), item.description.to_string()))
                    .collect();
                Self::vocabulary_table("Urges", &items)
            }
            UrgeResponse::UrgeDetails(UrgeDetailsResponse::V1(details)) => {
                let items = vec![(
                    details.urge.name.to_string(),
                    details.urge.description.to_string(),
                )];
                Self::vocabulary_table("Urge", &items)
            }
            UrgeResponse::NoUrges => Self::vocabulary_table("Urges", &[]),
            UrgeResponse::UrgeSet(UrgeSetResponse::V1(set)) => {
                McpResponse::new(format!("Urge set: {}", set.urge.name))
            }
            UrgeResponse::UrgeRemoved(UrgeRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Urge removed: {}", removed.name))
            }
        }
    }

    fn vocabulary_table(title: &str, items: &[(String, String)]) -> McpResponse {
        let mut md = format!("# {title}\n\n");
        if items.is_empty() {
            md.push_str(&format!("No {title} configured.\n"));
        } else {
            md.push_str("| Name | Description |\n");
            md.push_str("|------|-------------|\n");
            for (name, desc) in items {
                md.push_str(&format!("| {name} | {desc} |\n"));
            }
        }
        McpResponse::new(md).hint(Hint::inspect(ResourcePath::Agents.uri(), "View all agents"))
    }

    pub fn render(self) -> Rendered<UrgeResponse> {
        match self.response {
            UrgeResponse::UrgeSet(UrgeSetResponse::V1(set)) => {
                let prompt =
                    Confirmation::new("Urge", set.urge.name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("urge".to_string()).build(),
                );
                Rendered::new(
                    UrgeResponse::UrgeSet(UrgeSetResponse::V1(set)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            UrgeResponse::UrgeDetails(UrgeDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.urge.name.to_string())
                    .field("description:", details.urge.description.to_string())
                    .field("prompt:", details.urge.prompt.to_string())
                    .to_string();
                Rendered::new(
                    UrgeResponse::UrgeDetails(UrgeDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            UrgeResponse::Urges(UrgesResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("description", "Description").max(60),
                ]);
                for urge in &listed.items {
                    table.push_row(vec![urge.name.to_string(), urge.description.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    UrgeResponse::Urges(UrgesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            UrgeResponse::NoUrges => Rendered::new(
                UrgeResponse::NoUrges,
                format!("{}", "No urges configured.".muted()),
                String::new(),
            ),
            UrgeResponse::UrgeRemoved(UrgeRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Urge", removed.name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder().kind("urge".to_string()).build(),
                );
                Rendered::new(
                    UrgeResponse::UrgeRemoved(UrgeRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
