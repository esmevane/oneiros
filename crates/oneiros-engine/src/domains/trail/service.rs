use crate::*;

pub(crate) struct TrailService;

impl TrailService {
    /// Events that touched the given entity, oldest first.
    pub(crate) async fn of(
        scope: &Scope<AtBookmark>,
        entity_ref: &RefToken,
    ) -> Result<TrailResponse, TrailError> {
        let items = TrailRepo::new(scope).events_for(entity_ref).await?;
        if items.is_empty() {
            return Ok(TrailResponse::NoTrail);
        }
        let total = items.len();
        Ok(TrailResponse::TrailEvents(
            TrailEventsResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }

    /// Entity refs the given event emitted.
    pub(crate) async fn from(
        scope: &Scope<AtBookmark>,
        event_id: EventId,
    ) -> Result<TrailResponse, TrailError> {
        let items = TrailRepo::new(scope).refs_from(event_id).await?;
        if items.is_empty() {
            return Ok(TrailResponse::NoTrail);
        }
        let total = items.len();
        Ok(TrailResponse::EmittedRefs(
            EmittedRefsResponse::builder_v1()
                .items(items)
                .total(total)
                .build()
                .into(),
        ))
    }
}
