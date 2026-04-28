use crate::*;

pub struct SensationView {
    response: SensationResponse,
}

impl SensationView {
    pub fn new(response: SensationResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            SensationResponse::Sensations(SensationsResponse::V1(listed)) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|item| (item.name.to_string(), item.description.to_string()))
                    .collect();
                Self::vocabulary_table("Sensations", &items)
            }
            SensationResponse::SensationDetails(SensationDetailsResponse::V1(details)) => {
                let items = vec![(
                    details.sensation.name.to_string(),
                    details.sensation.description.to_string(),
                )];
                Self::vocabulary_table("Sensation", &items)
            }
            SensationResponse::NoSensations => Self::vocabulary_table("Sensations", &[]),
            SensationResponse::SensationSet(SensationSetResponse::V1(set)) => {
                McpResponse::new(format!("Sensation set: {}", set.sensation.name))
            }
            SensationResponse::SensationRemoved(SensationRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Sensation removed: {}", removed.name))
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

    pub fn render(self) -> Rendered<SensationResponse> {
        match self.response {
            SensationResponse::SensationSet(SensationSetResponse::V1(set)) => {
                let prompt = Confirmation::new("Sensation", set.sensation.name.to_string(), "set")
                    .to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("sensation".to_string())
                        .build(),
                );
                Rendered::new(
                    SensationResponse::SensationSet(SensationSetResponse::V1(set)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            SensationResponse::SensationDetails(SensationDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.sensation.name.to_string())
                    .field("description:", details.sensation.description.to_string())
                    .field("prompt:", details.sensation.prompt.to_string())
                    .to_string();
                Rendered::new(
                    SensationResponse::SensationDetails(SensationDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            SensationResponse::Sensations(SensationsResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("description", "Description").max(60),
                ]);
                for item in &listed.items {
                    table.push_row(vec![item.name.to_string(), item.description.to_string()]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    SensationResponse::Sensations(SensationsResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            SensationResponse::NoSensations => Rendered::new(
                SensationResponse::NoSensations,
                format!("{}", "No sensations configured.".muted()),
                String::new(),
            ),
            SensationResponse::SensationRemoved(SensationRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Sensation", removed.name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("sensation".to_string())
                        .build(),
                );
                Rendered::new(
                    SensationResponse::SensationRemoved(SensationRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
