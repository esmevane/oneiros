use crate::*;

pub struct TextureView;

impl TextureView {
    pub fn table(items: &Listed<Response<Texture>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("name", "Name"),
            Column::key("description", "Description").max(60),
        ]);
        for wrapped in &items.items {
            table.push_row(vec![
                wrapped.data.name.to_string(),
                wrapped.data.description.to_string(),
            ]);
        }
        table
    }

    pub fn detail(item: &Texture) -> Detail {
        Detail::new(item.name.to_string())
            .field("description:", item.description.to_string())
            .field("prompt:", item.prompt.to_string())
    }

    pub fn confirmed(verb: &str, name: &TextureName) -> Confirmation {
        Confirmation::new("Texture", name.to_string(), verb)
    }
}
