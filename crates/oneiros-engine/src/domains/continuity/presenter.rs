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
    ///
    /// Lifecycle commands (dream, wake, introspect, sleep, emerge, reflect)
    /// carry compact pressure summaries in response meta — ambient awareness
    /// of cognitive state without needing a separate pressure query.
    pub fn render(self) -> Rendered<Responses> {
        let summaries = self.pressure_summaries();
        let mut data = Response::new(Responses::from(self.response.clone()));

        if !summaries.is_empty() {
            let meta = data.meta.get_or_insert_with(ResponseMeta::default);
            meta.pressures = summaries;
        }

        let prompt = self.render_prompt();
        let text = self.render_text();

        Rendered::new(data, prompt, text)
    }

    /// Extract compact pressure summaries from lifecycle responses.
    fn pressure_summaries(&self) -> Vec<PressureSummary> {
        let context = match &self.response {
            ContinuityResponse::Dreaming(ctx)
            | ContinuityResponse::Waking(ctx)
            | ContinuityResponse::Emerged(ctx)
            | ContinuityResponse::Introspecting(ctx)
            | ContinuityResponse::Reflecting(ctx)
            | ContinuityResponse::Sleeping(ctx) => Some(ctx),
            _ => None,
        };

        context
            .map(|ctx| {
                ctx.pressures
                    .iter()
                    .map(|r| PressureSummary::from(&r.pressure))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn render_prompt(&self) -> String {
        match &self.response {
            ContinuityResponse::Dreaming(context)
            | ContinuityResponse::Waking(context)
            | ContinuityResponse::Emerged(context) => DreamTemplate::new(context).to_string(),
            ContinuityResponse::Status(table) => table.to_string(),
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
            ContinuityResponse::Status(table) => {
                format!("{} agents.", table.agents.len())
            }
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
