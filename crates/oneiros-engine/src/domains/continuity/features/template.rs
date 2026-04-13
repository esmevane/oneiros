use askama::Template;

use crate::*;

#[derive(Template)]
#[template(path = "continuity/dream.md")]
pub(crate) struct DreamTemplate<'a> {
    pub(crate) context: &'a DreamContext,
    pub(crate) pressures: RelevantPressures,
    pub(crate) readings: &'a [PressureReading],
    pub(crate) deep: bool,
}

impl<'a> DreamTemplate<'a> {
    pub(crate) fn new(context: &'a DreamContext) -> Self {
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

    pub(crate) fn deep(context: &'a DreamContext) -> Self {
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

    pub(crate) fn texture_names(&self) -> String {
        self.context
            .textures
            .iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn level_names(&self) -> String {
        self.context
            .levels
            .iter()
            .map(|l| l.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn sensation_names(&self) -> String {
        self.context
            .sensations
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn nature_names(&self) -> String {
        self.context
            .natures
            .iter()
            .map(|n| n.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn urge_names(&self) -> String {
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
pub(crate) struct IntrospectTemplate<'a> {
    pub(crate) agent: &'a Agent,
    pub(crate) pressures: RelevantPressures,
}

impl<'a> IntrospectTemplate<'a> {
    pub(crate) fn new(agent: &'a Agent, pressures: RelevantPressures) -> Self {
        Self { agent, pressures }
    }
}

#[derive(Template)]
#[template(path = "continuity/reflect.md")]
pub(crate) struct ReflectTemplate<'a> {
    pub(crate) agent: &'a Agent,
    pub(crate) pressures: RelevantPressures,
}

impl<'a> ReflectTemplate<'a> {
    pub(crate) fn new(agent: &'a Agent, pressures: RelevantPressures) -> Self {
        Self { agent, pressures }
    }
}

#[derive(Template)]
#[template(path = "continuity/sense.md")]
pub(crate) struct SenseTemplate<'a> {
    pub(crate) agent: &'a Agent,
    pub(crate) event_data: &'a str,
    pub(crate) pressures: RelevantPressures,
}

impl<'a> SenseTemplate<'a> {
    pub(crate) fn new(agent: &'a Agent, event_data: &'a str, pressures: RelevantPressures) -> Self {
        Self {
            agent,
            event_data,
            pressures,
        }
    }
}

#[derive(Template)]
#[template(path = "continuity/guidebook.md")]
pub(crate) struct GuidebookTemplate<'a> {
    pub(crate) context: &'a DreamContext,
}

impl<'a> GuidebookTemplate<'a> {
    pub(crate) fn new(context: &'a DreamContext) -> Self {
        Self { context }
    }
}
