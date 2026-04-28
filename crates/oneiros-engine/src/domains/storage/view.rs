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
            StorageResponse::StorageSet(StorageSetResponse::V1(set)) => {
                let prompt =
                    Confirmation::new("Storage", set.entry.key.to_string(), "stored").to_string();
                let ref_token = RefToken::new(Ref::storage(set.entry.key.clone()));
                let hints =
                    HintSet::mutation(MutationHints::builder().ref_token(ref_token).build());
                Rendered::new(
                    StorageResponse::StorageSet(StorageSetResponse::V1(set)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            StorageResponse::StorageDetails(StorageDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.entry.key.to_string())
                    .field("description:", details.entry.description.to_string())
                    .field("hash:", details.entry.hash.to_string())
                    .to_string();
                Rendered::new(
                    StorageResponse::StorageDetails(StorageDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            StorageResponse::Entries(StorageEntriesResponse::V1(listed)) => {
                let mut table = Table::new(vec![
                    Column::key("key", "Key"),
                    Column::key("description", "Description").max(40),
                    Column::key("hash", "Hash"),
                ]);
                for entry in &listed.items {
                    table.push_row(vec![
                        entry.key.to_string(),
                        entry.description.to_string(),
                        entry.hash.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(
                    StorageResponse::Entries(StorageEntriesResponse::V1(listed)),
                    prompt,
                    String::new(),
                )
            }
            StorageResponse::NoEntries => Rendered::new(
                StorageResponse::NoEntries,
                format!("{}", "No storage entries.".muted()),
                String::new(),
            ),
            StorageResponse::StorageRemoved(StorageRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Storage", removed.key.to_string(), "removed").to_string();
                Rendered::new(
                    StorageResponse::StorageRemoved(StorageRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
