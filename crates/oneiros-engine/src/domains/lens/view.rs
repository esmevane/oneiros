use crate::*;

pub(crate) struct LensView {
    response: LensResponse,
}

impl LensView {
    pub(crate) fn new(response: LensResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<LensResponse> {
        let prompt = match &self.response {
            LensResponse::Parsed(ParsedLensResponse::V1(parsed)) => {
                format!(
                    "{}\n{}",
                    format!("source: {}", parsed.source).muted(),
                    parsed.display,
                )
            }
            LensResponse::Explained(ExplainedLensResponse::V1(explained)) => {
                format!(
                    "{}\n{}\n\n{}",
                    format!("source: {}", explained.source).muted(),
                    explained.display,
                    explained.plan,
                )
            }
            LensResponse::Queried(QueriedLensResponse::V1(queried)) => {
                if queried.hits.is_empty() {
                    format!("No results for '{}'.", queried.source)
                } else {
                    let mut table = Table::new(vec![
                        Column::new("Kind"),
                        Column::new("Ref"),
                        Column::new("Timestamp"),
                    ]);
                    for hit in &queried.hits {
                        match hit {
                            Hit::Entity(e) => {
                                table.push_row(vec![
                                    "entity".to_string(),
                                    RefToken::new(e.entity_ref.clone()).to_string(),
                                    e.timestamp.to_string(),
                                ]);
                            }
                            Hit::Event(e) => {
                                table.push_row(vec![
                                    "event".to_string(),
                                    e.event_id.to_string(),
                                    e.timestamp.to_string(),
                                ]);
                            }
                        }
                    }
                    let count_text = format!(
                        "{} result{}",
                        queried.total,
                        if queried.total == 1 { "" } else { "s" }
                    );
                    let summary = count_text.muted();
                    format!("{summary}\n\n{table}")
                }
            }
        };
        Rendered::new(self.response, prompt, String::new())
    }
}
