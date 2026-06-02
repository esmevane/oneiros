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
                    &format!("created ({count} events)", count = created.slice.event_count),
                )
                .to_string();
                Rendered::new(
                    SliceResponse::Created(SliceCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
            }
            other => Rendered::new(other, "slice".to_string(), String::new()),
        }
    }
}
