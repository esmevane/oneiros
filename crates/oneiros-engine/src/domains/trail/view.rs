//! Trail view — presentation authority for the trail domain.

use crate::*;

pub(crate) struct TrailView {
    response: TrailResponse,
}

impl TrailView {
    pub(crate) fn new(response: TrailResponse) -> Self {
        Self { response }
    }

    pub(crate) fn render(self) -> Rendered<TrailResponse> {
        match self.response {
            TrailResponse::TrailEvents(TrailEventsResponse::V1(events)) => {
                let prompt = format!(
                    "{}\n\n{}",
                    format_args!("{} event(s) touched this entity", events.items.len()).muted(),
                    Self::events_table(&events.items),
                );
                Rendered::new(
                    TrailResponse::TrailEvents(TrailEventsResponse::V1(events)),
                    prompt,
                    String::new(),
                )
            }
            TrailResponse::EmittedRefs(EmittedRefsResponse::V1(refs)) => {
                let prompt = format!(
                    "{}\n\n{}",
                    format_args!("{} entity(ies) emitted by this event", refs.items.len()).muted(),
                    Self::refs_table(&refs.items),
                );
                Rendered::new(
                    TrailResponse::EmittedRefs(EmittedRefsResponse::V1(refs)),
                    prompt,
                    String::new(),
                )
            }
            TrailResponse::NoTrail => Rendered::new(
                TrailResponse::NoTrail,
                "No trail.".muted().to_string(),
                String::new(),
            ),
        }
    }

    fn events_table(items: &[TrailEntry]) -> Table {
        let mut table = Table::new(vec![
            Column::new("Event"),
            Column::new("Type"),
            Column::new("At"),
        ]);
        for entry in items {
            table.push_row(vec![
                entry.event_id.to_string(),
                entry.event_type.clone(),
                entry.created_at.as_string(),
            ]);
        }
        table
    }

    fn refs_table(items: &[RefToken]) -> Table {
        let mut table = Table::new(vec![Column::new("Ref")]);
        for entity_ref in items {
            table.push_row(vec![entity_ref.to_string()]);
        }
        table
    }
}
