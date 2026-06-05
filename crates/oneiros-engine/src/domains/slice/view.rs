use crate::*;

pub(crate) struct SliceView {
    response: SliceResponse,
}

impl SliceView {
    pub(crate) fn new(response: SliceResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<SliceResponse> {
        match self.response {
            SliceResponse::Created(SliceCreatedResponse::V1(created)) => {
                let prompt = Confirmation::new(
                    "Slice",
                    created.slice.name.to_string(),
                    format!(
                        "created ({count} events)",
                        count = created.slice.event_count
                    ),
                )
                .to_string();
                Rendered::new(
                    SliceResponse::Created(SliceCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            SliceResponse::Slices(ref listed) => {
                let mut table = Table::new(vec![
                    Column::new("Name"),
                    Column::new("Lens").max(50),
                    Column::new("Events"),
                ]);
                for item in &listed.items {
                    table.push_row(vec![
                        item.name.to_string(),
                        item.lens_expr.clone(),
                        item.event_count.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.items.len(), listed.total).muted(),
                );
                Rendered::new(self.response, prompt, String::new())
            }
            SliceResponse::Deleted(SliceDeletedResponse::V1(deleted)) => {
                let prompt =
                    Confirmation::new("Slice", deleted.name.to_string(), "deleted").to_string();
                Rendered::new(
                    SliceResponse::Deleted(SliceDeletedResponse::V1(deleted)),
                    prompt,
                    String::new(),
                )
            }
            SliceResponse::Diffed(SliceDiffedResponse::V1(diffed)) => {
                let prompt = format!(
                    "Slice diff: {} only in source, {} only in target, {} in both",
                    diffed.only_in_source, diffed.only_in_target, diffed.in_both,
                );
                Rendered::new(
                    SliceResponse::Diffed(SliceDiffedResponse::V1(diffed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
