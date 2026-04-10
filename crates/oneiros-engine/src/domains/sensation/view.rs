use crate::*;

pub struct SensationView;

impl SensationView {
    pub fn table(items: &Listed<Response<Sensation>>) -> Table {
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

    pub fn detail(item: &Sensation) -> Detail {
        Detail::new(item.name.to_string())
            .field("description:", item.description.to_string())
            .field("prompt:", item.prompt.to_string())
    }

    pub fn confirmed(verb: &str, name: &SensationName) -> Confirmation {
        Confirmation::new("Sensation", name.to_string(), verb)
    }
}
