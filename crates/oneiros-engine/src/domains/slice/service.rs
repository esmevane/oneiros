use crate::*;
use std::collections::HashSet;

pub(crate) struct SliceService;

impl SliceService {
    /// Creates a slice by evaluating the lens expression against the
    /// project's event log, emitting a `SliceCreated` event to the host
    /// log, and waiting for the host projection to materialize.
    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        project_scope: &Scope<AtBookmark>,
        mailbox: &Mailbox,
        canons: &CanonIndex,
        request: &CreateSlice,
    ) -> Result<SliceResponse, SliceError> {
        let CreateSlice::V1(req) = request;

        let event_lens = format!("events_for({})", req.lens_expr);
        let selection = LensService::select(project_scope, canons, &event_lens).await?;
        let event_count = selection.event_ids().len() as u64;

        let slice = Slice::builder()
            .name(req.name.clone())
            .lens_expr(req.lens_expr.clone())
            .event_count(event_count)
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Slice(SliceEvents::SliceCreated(
                SliceCreated::builder_v1().slice(slice).build().into(),
            )))
            .build();
        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        let projected = SliceRepo::new(scope)
            .fetch(&req.name)
            .await?
            .ok_or(SliceError::NotFound(req.name.clone()))?;

        Ok(SliceResponse::Created(
            SliceCreatedResponse::builder_v1()
                .slice(projected)
                .build()
                .into(),
        ))
    }

    /// Lists all slices from the host DB.
    pub(crate) async fn list(scope: &Scope<AtHost>) -> Result<SliceResponse, SliceError> {
        let listed = SliceRepo::new(scope).list().await?;
        Ok(SliceResponse::Slices(listed))
    }

    /// Deletes a slice by name. Emits a `SliceDeleted` event to the host log.
    pub(crate) async fn delete(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        name: &SliceName,
    ) -> Result<SliceResponse, SliceError> {
        let deleted = SliceDeleted::builder_v1().name(name.clone()).build().into();
        let new_event = NewEvent::builder()
            .data(Events::Slice(SliceEvents::SliceDeleted(deleted)))
            .build();
        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        scope
            .config()
            .fetch
            .until_absent(|| async { SliceRepo::new(scope).get(name).await })
            .await
            .map_err(|_| SliceError::NotFound(name.clone()))?;

        Ok(SliceResponse::Deleted(
            SliceDeletedResponse::builder_v1()
                .id(SliceId::default())
                .name(name.clone())
                .build()
                .into(),
        ))
    }

    /// Diffs two slices by re-evaluating their lens expressions and
    /// comparing the resulting event ID sets.
    pub(crate) async fn diff(
        scope: &Scope<AtHost>,
        project_scope: &Scope<AtBookmark>,
        canons: &CanonIndex,
        source: &SliceName,
        target: &SliceName,
    ) -> Result<SliceResponse, SliceError> {
        let source_slice = SliceRepo::new(scope)
            .get(source)
            .await?
            .ok_or(SliceError::NotFound(source.clone()))?;
        let target_slice = SliceRepo::new(scope)
            .get(target)
            .await?
            .ok_or(SliceError::NotFound(target.clone()))?;

        let source_ids = Self::event_ids(project_scope, canons, &source_slice.lens_expr).await?;
        let target_ids = Self::event_ids(project_scope, canons, &target_slice.lens_expr).await?;

        let only_in_source = source_ids.difference(&target_ids).count() as u64;
        let only_in_target = target_ids.difference(&source_ids).count() as u64;
        let in_both = source_ids.intersection(&target_ids).count() as u64;

        Ok(SliceResponse::Diffed(
            SliceDiffedResponse::builder_v1()
                .only_in_source(only_in_source)
                .only_in_target(only_in_target)
                .in_both(in_both)
                .build()
                .into(),
        ))
    }

    /// Evaluates a lens expression and returns the set of matching event IDs.
    async fn event_ids(
        project_scope: &Scope<AtBookmark>,
        canons: &CanonIndex,
        lens_expr: &str,
    ) -> Result<HashSet<EventId>, SliceError> {
        let event_lens = format!("events_for({})", lens_expr);
        let selection = LensService::select(project_scope, canons, &event_lens).await?;
        Ok(selection.event_ids().into_iter().collect())
    }
}
