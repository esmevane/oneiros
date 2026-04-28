use crate::*;

pub struct TextureView {
    response: TextureResponse,
}

impl TextureView {
    pub fn new(response: TextureResponse) -> Self {
        Self { response }
    }

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            TextureResponse::Textures(TexturesResponse::V1(listed)) => {
                let items: Vec<_> = listed
                    .items
                    .iter()
                    .map(|item| (item.name.to_string(), item.description.to_string()))
                    .collect();
                Self::vocabulary_table("Textures", &items)
            }
            TextureResponse::TextureDetails(TextureDetailsResponse::V1(details)) => {
                let items = vec![(
                    details.texture.name.to_string(),
                    details.texture.description.to_string(),
                )];
                Self::vocabulary_table("Texture", &items)
            }
            TextureResponse::NoTextures => Self::vocabulary_table("Textures", &[]),
            TextureResponse::TextureSet(TextureSetResponse::V1(set)) => {
                McpResponse::new(format!("Texture set: {}", set.texture.name))
            }
            TextureResponse::TextureRemoved(TextureRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Texture removed: {}", removed.name))
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

    pub fn render(self) -> Rendered<TextureResponse> {
        match self.response {
            TextureResponse::TextureSet(TextureSetResponse::V1(set)) => {
                let prompt =
                    Confirmation::new("Texture", set.texture.name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("texture".to_string())
                        .build(),
                );
                Rendered::new(
                    TextureResponse::TextureSet(TextureSetResponse::V1(set)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            TextureResponse::TextureDetails(TextureDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.texture.name.to_string())
                    .field("description:", details.texture.description.to_string())
                    .field("prompt:", details.texture.prompt.to_string())
                    .to_string();
                Rendered::new(
                    TextureResponse::TextureDetails(TextureDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            TextureResponse::Textures(TexturesResponse::V1(listed)) => {
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
                    TextureResponse::Textures(TexturesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            TextureResponse::NoTextures => Rendered::new(
                TextureResponse::NoTextures,
                format!("{}", "No textures configured.".muted()),
                String::new(),
            ),
            TextureResponse::TextureRemoved(TextureRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Texture", removed.name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("texture".to_string())
                        .build(),
                );
                Rendered::new(
                    TextureResponse::TextureRemoved(TextureRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
        }
    }
}
