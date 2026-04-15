use crate::*;

pub struct TextureView {
    response: TextureResponse,
}

impl TextureView {
    pub fn new(response: TextureResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<TextureResponse> {
        match self.response {
            TextureResponse::TextureSet(name) => {
                let prompt = Confirmation::new("Texture", name.to_string(), "set").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("texture".to_string())
                        .build(),
                );
                Rendered::new(TextureResponse::TextureSet(name), prompt, String::new())
                    .with_hints(hints)
            }
            TextureResponse::TextureDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.name.to_string())
                    .field("description:", wrapped.data.description.to_string())
                    .field("prompt:", wrapped.data.prompt.to_string())
                    .to_string();
                Rendered::new(
                    TextureResponse::TextureDetails(wrapped),
                    prompt,
                    String::new(),
                )
            }
            TextureResponse::Textures(listed) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("description", "Description").max(60),
                ]);
                for wrapped in &listed.items {
                    table.push_row(vec![
                        wrapped.data.name.to_string(),
                        wrapped.data.description.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(TextureResponse::Textures(listed), prompt, String::new())
            }
            TextureResponse::NoTextures => Rendered::new(
                TextureResponse::NoTextures,
                format!("{}", "No textures configured.".muted()),
                String::new(),
            ),
            TextureResponse::TextureRemoved(name) => {
                let prompt = Confirmation::new("Texture", name.to_string(), "removed").to_string();
                let hints = HintSet::vocabulary(
                    VocabularyHints::builder()
                        .kind("texture".to_string())
                        .build(),
                );
                Rendered::new(TextureResponse::TextureRemoved(name), prompt, String::new())
                    .with_hints(hints)
            }
        }
    }
}
