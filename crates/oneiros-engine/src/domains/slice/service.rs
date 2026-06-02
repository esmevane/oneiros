use crate::*;

pub(crate) struct SliceService;

impl SliceService {
    /// Creates a slice by evaluating the lens expression against the
    /// project's event log. The lens is an entity-level expression;
    /// the service wraps it with `events_for()` to find matching events.
    pub(crate) async fn create(
        scope: &Scope<AtBookmark>,
        canons: &CanonIndex,
        request: &CreateSlice,
    ) -> Result<SliceResponse, SliceError> {
        let CreateSlice::V1(req) = request;

        let event_lens = format!("events_for({})", req.lens_expr);
        let selection = LensService::select(scope, canons, &event_lens).await?;

        let event_count = selection.event_ids().len() as u64;

        let slice = Slice::builder()
            .name(req.name.clone())
            .lens_expr(req.lens_expr.clone())
            .event_count(event_count)
            .build();

        Ok(SliceResponse::Created(
            SliceCreatedResponse::builder_v1()
                .slice(slice)
                .build()
                .into(),
        ))
    }
}
