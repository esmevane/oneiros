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
            ContinuityResponse::Dreaming(ctx)
            | ContinuityResponse::Waking(ctx)
            | ContinuityResponse::Emerged(ctx)
            | ContinuityResponse::Status(ctx) => DreamTemplate::new(ctx).to_string(),
            ContinuityResponse::Introspecting(ctx) => {
                let pressures = Self::relevant_pressures(ctx);
                IntrospectTemplate::new(&ctx.agent, pressures).to_string()
            }
            ContinuityResponse::Reflecting(ctx) => {
                let pressures = Self::relevant_pressures(ctx);
                ReflectTemplate::new(&ctx.agent, pressures).to_string()
            }
            ContinuityResponse::Sleeping(ctx) => {
                let pressures = Self::relevant_pressures(ctx);
                IntrospectTemplate::new(&ctx.agent, pressures).to_string()
            }
            ContinuityResponse::Guidebook(ctx) => GuidebookTemplate::new(ctx).to_string(),
            ContinuityResponse::Receded(_) => String::new(),
        }
    }

    fn render_text(&self) -> String {
        match &self.response {
            ContinuityResponse::Waking(ctx) => format!("Waking as {}...", ctx.agent.name),
            ContinuityResponse::Dreaming(ctx) => format!("Dreaming as {}...", ctx.agent.name),
            ContinuityResponse::Emerged(ctx) => format!("Emerged as {}.", ctx.agent.name),
            ContinuityResponse::Introspecting(ctx) => {
                format!("Introspecting as {}...", ctx.agent.name)
            }
            ContinuityResponse::Reflecting(ctx) => {
                format!("Reflecting as {}...", ctx.agent.name)
            }
            ContinuityResponse::Sleeping(ctx) => format!("Sleeping as {}...", ctx.agent.name),
            ContinuityResponse::Guidebook(ctx) => format!("Guidebook for {}.", ctx.agent.name),
            ContinuityResponse::Receded(name) => format!("Agent {} has receded.", name),
            ContinuityResponse::Status(ctx) => format!("Status for {}.", ctx.agent.name),
        }
    }

    fn relevant_pressures(ctx: &DreamContext) -> RelevantPressures {
        RelevantPressures::from_pressures(
            ctx.pressures.iter().map(|r| r.pressure.clone()).collect(),
        )
    }
}
