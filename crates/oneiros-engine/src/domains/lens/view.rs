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
        };
        Rendered::new(self.response, prompt, String::new())
    }
}
