use askama::Template;

use crate::*;

/// How many recent cognitions/experiences/threads the dream surfaces inline.
/// The substrate carries the rest — agents reach for it via tools after waking.
const LATEST_LIMIT: usize = 3;

#[derive(Template)]
#[template(path = "continuity/dream.md")]
pub struct DreamTemplate<'a> {
    pub context: &'a DreamContext,
    pub pressures: RelevantPressures,
    pub readings: &'a [PressureReading],
}

impl<'a> DreamTemplate<'a> {
    pub fn new(context: &'a DreamContext) -> Self {
        let pressures = RelevantPressures::from_pressures(
            context
                .pressures
                .iter()
                .map(|r| r.pressure.clone())
                .collect(),
        );
        Self {
            context,
            pressures,
            readings: &context.pressures,
        }
    }

    pub fn today(&self) -> String {
        chrono::Utc::now().date_naive().to_string()
    }

    pub fn core_memories(&self) -> Vec<&Memory> {
        self.context
            .memories
            .iter()
            .filter(|m| m.level.as_str() == "core")
            .collect()
    }

    pub fn latest_cognitions(&self) -> Vec<&Cognition> {
        Self::tail(&self.context.cognitions, LATEST_LIMIT)
    }

    pub fn latest_experiences(&self) -> Vec<&Experience> {
        Self::tail(&self.context.experiences, LATEST_LIMIT)
    }

    pub fn latest_threads(&self) -> Vec<&Connection> {
        Self::tail(&self.context.connections, LATEST_LIMIT)
    }

    fn tail<T>(items: &[T], limit: usize) -> Vec<&T> {
        let len = items.len();
        let start = len.saturating_sub(limit);
        items[start..].iter().collect()
    }
}

#[derive(Template)]
#[template(path = "continuity/introspect.md")]
pub struct IntrospectTemplate<'a> {
    pub agent: &'a Agent,
    pub pressures: RelevantPressures,
}

impl<'a> IntrospectTemplate<'a> {
    pub fn new(agent: &'a Agent, pressures: RelevantPressures) -> Self {
        Self { agent, pressures }
    }
}

#[derive(Template)]
#[template(path = "continuity/reflect.md")]
pub struct ReflectTemplate<'a> {
    pub agent: &'a Agent,
    pub pressures: RelevantPressures,
}

impl<'a> ReflectTemplate<'a> {
    pub fn new(agent: &'a Agent, pressures: RelevantPressures) -> Self {
        Self { agent, pressures }
    }
}

#[derive(Template)]
#[template(path = "continuity/sense.md")]
pub struct SenseTemplate<'a> {
    pub agent: &'a Agent,
    pub event_data: &'a str,
    pub pressures: RelevantPressures,
}

impl<'a> SenseTemplate<'a> {
    pub fn new(agent: &'a Agent, event_data: &'a str, pressures: RelevantPressures) -> Self {
        Self {
            agent,
            event_data,
            pressures,
        }
    }
}

#[derive(Template)]
#[template(path = "continuity/guidebook.md")]
pub struct GuidebookTemplate<'a> {
    pub context: &'a DreamContext,
}

impl<'a> GuidebookTemplate<'a> {
    pub fn new(context: &'a DreamContext) -> Self {
        Self { context }
    }
}
