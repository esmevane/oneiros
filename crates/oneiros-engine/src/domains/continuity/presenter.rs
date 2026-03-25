//! Continuity presenter — knows how to represent continuity responses.
//!
//! Always produces the richest representation available. The caller
//! decides at the last mile which form to consume (data, prompt, text)
//! via the Rendered accessors.

use crate::*;

pub struct ContinuityPresenter {
    response: ContinuityResponse,
}

impl ContinuityPresenter {
    pub fn new(response: ContinuityResponse) -> Self {
        Self { response }
    }

    /// Render this continuity response into all available forms.
    pub fn render(self) -> Rendered<Responses> {
        let data = Response::new(Responses::from(self.response.clone()));
        let prompt = self.render_prompt();
        let text = self.render_text();

        Rendered::new(data, prompt, text)
    }

    fn render_prompt(&self) -> String {
        match &self.response {
            ContinuityResponse::Dreaming(context)
            | ContinuityResponse::Waking(context)
            | ContinuityResponse::Emerged(context)
            | ContinuityResponse::Status(context) => DreamTemplate::new(context).to_string(),
            ContinuityResponse::Introspecting(context) => {
                let pressures = Self::relevant_pressures(context);
                IntrospectTemplate::new(&context.agent, pressures).to_string()
            }
            ContinuityResponse::Reflecting(context) => {
                let pressures = Self::relevant_pressures(context);
                ReflectTemplate::new(&context.agent, pressures).to_string()
            }
            ContinuityResponse::Sleeping(context) => {
                let pressures = Self::relevant_pressures(context);
                IntrospectTemplate::new(&context.agent, pressures).to_string()
            }
            ContinuityResponse::Guidebook(context) => GuidebookTemplate::new(context).to_string(),
            ContinuityResponse::Receded(name) => format!(
                "Agent '{}' has receded. Their cognitions, memories, and experiences remain in the record, but they will no longer participate in active sessions.",
                name
            ),
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            ContinuityResponse::Waking(context) => format!("Waking as {}...", context.agent.name),
            ContinuityResponse::Dreaming(context) => {
                format!("Dreaming as {}...", context.agent.name)
            }
            ContinuityResponse::Emerged(context) => format!("Emerged as {}.", context.agent.name),
            ContinuityResponse::Introspecting(context) => {
                format!("Introspecting as {}...", context.agent.name)
            }
            ContinuityResponse::Reflecting(context) => {
                format!("Reflecting as {}...", context.agent.name)
            }
            ContinuityResponse::Sleeping(context) => {
                format!("Sleeping as {}...", context.agent.name)
            }
            ContinuityResponse::Guidebook(context) => {
                format!("Guidebook for {}.", context.agent.name)
            }
            ContinuityResponse::Receded(name) => format!("Agent {} has receded.", name),
            ContinuityResponse::Status(context) => format!("Status for {}.", context.agent.name),
        }
    }

    fn relevant_pressures(context: &DreamContext) -> RelevantPressures {
        RelevantPressures::from_pressures(
            context
                .pressures
                .iter()
                .map(|r| r.pressure.clone())
                .collect(),
        )
    }
}
