//! Storage view — presentation authority for the storage domain.

use crate::*;

pub struct StorageView {
    response: StorageResponse,
}

impl StorageView {
    pub fn new(response: StorageResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<StorageResponse> {
        match self.response {
            StorageResponse::StorageSet(wrapped) => {
                let prompt = Confirmation::new("Storage", wrapped.data.key().to_string(), "stored")
                    .to_string();
                let hints = match wrapped.meta().ref_token() {
                    Some(ref_token) => {
                        HintSet::mutation(MutationHints::builder().ref_token(ref_token).build())
                    }
                    None => HintSet::None,
                };
                Rendered::new(StorageResponse::StorageSet(wrapped), prompt, String::new())
                    .with_hints(hints)
            }
            StorageResponse::StorageDetails(wrapped) => {
                let prompt = Detail::new(wrapped.data.key().to_string())
                    .field("description:", wrapped.data.description().to_string())
                    .field("hash:", wrapped.data.hash().to_string())
                    .to_string();
                Rendered::new(
                    StorageResponse::StorageDetails(wrapped),
                    prompt,
                    String::new(),
                )
            }
            StorageResponse::Entries(listed) => {
                let mut table = Table::new(vec![
                    Column::key("key", "Key"),
                    Column::key("description", "Description").max(40),
                    Column::key("hash", "Hash"),
                ]);
                for wrapped in &listed.items {
                    table.push_row(vec![
                        wrapped.data.key().to_string(),
                        wrapped.data.description().to_string(),
                        wrapped.data.hash().to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(StorageResponse::Entries(listed), prompt, String::new())
            }
            StorageResponse::NoEntries => Rendered::new(
                StorageResponse::NoEntries,
                format!("{}", "No storage entries.".muted()),
                String::new(),
            ),
            StorageResponse::StorageRemoved(key) => {
                let prompt = Confirmation::new("Storage", key.to_string(), "removed").to_string();
                Rendered::new(StorageResponse::StorageRemoved(key), prompt, String::new())
            }
        }
    }
}
