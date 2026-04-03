use askama::Template;

use crate::*;

#[derive(Template)]
#[template(path = "continuity/dream.md")]
pub struct DreamTemplate<'a> {
    pub context: &'a DreamContext,
    pub pressures: RelevantPressures,
    pub readings: &'a [PressureReading],
    pub deep: bool,
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
            deep: false,
        }
    }

    pub fn deep(context: &'a DreamContext) -> Self {
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
            deep: true,
        }
    }

    pub fn texture_names(&self) -> String {
        self.context
            .textures
            .iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn level_names(&self) -> String {
        self.context
            .levels
            .iter()
            .map(|l| l.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn sensation_names(&self) -> String {
        self.context
            .sensations
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn nature_names(&self) -> String {
        self.context
            .natures
            .iter()
            .map(|n| n.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn urge_names(&self) -> String {
        self.context
            .urges
            .iter()
            .map(|u| u.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
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
