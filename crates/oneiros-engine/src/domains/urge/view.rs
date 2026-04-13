use crate::*;

pub(crate) struct UrgeView;

impl UrgeView {
    pub(crate) fn table(items: &Listed<Urge>) -> Table {
        let mut table = Table::new(vec![
            Column::key("name", "Name"),
            Column::key("description", "Description").max(60),
        ]);
        for urge in &items.items {
            table.push_row(vec![urge.name.to_string(), urge.description.to_string()]);
        }
        table
    }

    pub(crate) fn detail(item: &Urge) -> Detail {
        Detail::new(item.name.to_string())
            .field("description:", item.description.to_string())
            .field("prompt:", item.prompt.to_string())
    }

    pub(crate) fn confirmed(verb: &str, name: &UrgeName) -> Confirmation {
        Confirmation::new("Urge", name.to_string(), verb)
    }
}
