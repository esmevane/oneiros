use crate::*;

pub struct NatureView {
    response: NatureResponse,
}

impl NatureView {
    pub fn new(response: NatureResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            NatureResponse::Natures(NaturesResponse::V1(listed)) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|item| (item.name.to_string(), item.description.to_string()))
                    .collect();
                Self::vocabulary_table("Natures", &items)
            }
            NatureResponse::NatureDetails(NatureDetailsResponse::V1(details)) => {
                let items = vec![(
                    details.nature.name.to_string(),
                    details.nature.description.to_string(),
                )];
                Self::vocabulary_table("Nature", &items)
            }
            NatureResponse::NoNatures => Self::vocabulary_table("Natures", &[]),
            NatureResponse::NatureSet(NatureSetResponse::V1(set)) => {
                McpResponse::new(format!("Nature set: {}", set.nature.name))
            }
            NatureResponse::NatureRemoved(NatureRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Nature removed: {}", removed.name))
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

    pub fn render(self) -> Rendered<NatureResponse> {
        match self.response {
            NatureResponse::NatureSet(NatureSetResponse::V1(set)) => {
                let prompt =
                    Confirmation::new("Nature", set.nature.name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("nature".to_string())
                        .build(),
                );
                Rendered::new(
                    NatureResponse::NatureSet(NatureSetResponse::V1(set)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            NatureResponse::NatureDetails(NatureDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.nature.name.to_string())
                    .field("description:", details.nature.description.to_string())
                    .field("prompt:", details.nature.prompt.to_string())
                    .to_string();
                Rendered::new(
                    NatureResponse::NatureDetails(NatureDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            NatureResponse::Natures(NaturesResponse::V1(listed)) => {
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
                    NatureResponse::Natures(NaturesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            NatureResponse::NoNatures => Rendered::new(
                NatureResponse::NoNatures,
                format!("{}", "No natures configured.".muted()),
                String::new(),
            ),
            NatureResponse::NatureRemoved(NatureRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Nature", removed.name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("nature".to_string())
                        .build(),
                );
                Rendered::new(
                    NatureResponse::NatureRemoved(NatureRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
