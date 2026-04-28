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

    pub fn mcp(&self) -> McpResponse {
        match &self.response {
            ContinuityResponse::Status(StatusResponse::V1(details)) => {
                let table = &details.table;
                let mut md = String::from("# Agent Status\n\n");
                md.push_str("| Agent | Cognitions | Memories | Experiences |\n");
                md.push_str("|-------|------------|----------|-------------|\n");
                for row in &table.agents {
                    md.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        row.name, row.cognition_count, row.memory_count, row.experience_count,
                    ));
                }
                McpResponse::new(md)
                    .hint(Hint::inspect(
                        ResourcePath::Pressure.uri(),
                        "Check pressure across agents",
                    ))
                    .hint(Hint::inspect(ResourcePath::Agents.uri(), "View all agents"))
            }
            _ => McpResponse::new(String::new()),
        }
    }

    /// Render this continuity response into all available forms.
    pub fn render(self) -> Rendered<ContinuityResponse> {
        let (prompt, text, hints) = match &self.response {
            ContinuityResponse::Dreaming(DreamingResponse::V1(details)) => {
                let context = &details.context;
                let template = DreamTemplate::new(context).to_string();
                let text = format!("Dreaming as {}...", context.agent.name);
                (template, text, HintSet::None)
            }
            ContinuityResponse::Emerged(EmergedResponse::V1(details)) => {
                let context = &details.context;
                let template = DreamTemplate::new(context).to_string();
                let text = format!("Emerged as {}.", context.agent.name);
                (template, text, HintSet::None)
            }
            ContinuityResponse::Waking(WakingResponse::V1(details)) => {
                let context = &details.context;
                let template = DreamTemplate::new(context).to_string();
                let pressures = context
                    .pressures
                    .iter()
                    .map(|r| PressureSummary::from(&r.pressure))
                    .collect();
                let hints = HintSet::wake(
                    WakeHints::builder()
                        .agent(context.agent.name.clone())
                        .pressures(pressures)
                        .build(),
                );
                (
                    template,
                    format!("Waking as {}...", context.agent.name),
                    hints,
                )
            }
            ContinuityResponse::Status(StatusResponse::V1(details)) => {
                let table = &details.table;
                (
                    table.to_string(),
                    format!("{} agents.", table.agents.len()),
                    HintSet::None,
                )
            }
            ContinuityResponse::Introspecting(IntrospectingResponse::V1(details)) => {
                let context = &details.context;
                let pressures = Self::relevant_pressures(context);
                (
                    IntrospectTemplate::new(&context.agent, pressures).to_string(),
                    format!("Introspecting as {}...", context.agent.name),
                    HintSet::None,
                )
            }
            ContinuityResponse::Reflecting(ReflectingResponse::V1(details)) => {
                let context = &details.context;
                let pressures = Self::relevant_pressures(context);
                let hints = HintSet::reflect(
                    ReflectHints::builder()
                        .agent(context.agent.name.clone())
                        .build(),
                );
                (
                    ReflectTemplate::new(&context.agent, pressures).to_string(),
                    format!("Reflecting as {}...", context.agent.name),
                    hints,
                )
            }
            ContinuityResponse::Sleeping(SleepingResponse::V1(details)) => {
                let context = &details.context;
                let pressures = Self::relevant_pressures(context);
                (
                    IntrospectTemplate::new(&context.agent, pressures).to_string(),
                    format!("Sleeping as {}...", context.agent.name),
                    HintSet::None,
                )
            }
            ContinuityResponse::Guidebook(GuidebookResponse::V1(details)) => {
                let context = &details.context;
                (
                    GuidebookTemplate::new(context).to_string(),
                    format!("Guidebook for {}.", context.agent.name),
                    HintSet::None,
                )
            }
            ContinuityResponse::Receded(RecededResponse::V1(details)) => (
                format!(
                    "Agent '{}' has receded. Their cognitions, memories, and experiences remain in the record, but they will no longer participate in active sessions.",
                    details.agent
                ),
                format!("Agent {} has receded.", details.agent),
                HintSet::None,
            ),
        };

        Rendered::new(self.response, prompt, text).with_hints(hints)
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
