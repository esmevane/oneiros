//! `SliceActor` — singleton that evaluates slice lenses against new
//! events prospectively. When a new event is stored, the actor checks
//! whether it matches any slice's lens expression and emits a
//! `SliceMatched` event for each match.
//!
//! Receives `ProjectMessage::SliceMatch`; no-ops other variants.

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct ProjectSliceMailbox {
    tx: mpsc::UnboundedSender<ProjectMessage>,
}

impl ProjectSliceMailbox {
    pub(crate) fn open() -> (Self, ProjectSliceInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, ProjectSliceInbox { rx })
    }

    pub(crate) fn tell(&self, message: ProjectMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(
                error = %error,
                "project slice: receiver closed; message dropped"
            );
        }
    }
}

pub(crate) struct ProjectSliceInbox {
    rx: mpsc::UnboundedReceiver<ProjectMessage>,
}

impl ProjectSliceInbox {
    pub(crate) async fn recv(&mut self) -> Option<ProjectMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct SliceActor {
    mailbox: Mailbox,
    canons: CanonIndex,
}

impl SliceActor {
    pub(crate) fn spawn(inbox: ProjectSliceInbox, mailbox: Mailbox, canons: CanonIndex) {
        tokio::spawn(Self { mailbox, canons }.run(inbox));
    }

    async fn run(self, mut inbox: ProjectSliceInbox) {
        while let Some(message) = inbox.recv().await {
            if let ProjectMessage::SliceMatch(slice_match) = message {
                if let Err(error) = self.handle_match(slice_match).await {
                    tracing::error!(?error, "slice actor: match handling failed");
                }
            }
        }
    }

    async fn handle_match(&self, msg: SliceMatch) -> Result<(), EventError> {
        let scope = &msg.scope;
        let stored = &msg.stored;

        let slices = SliceRepo::new(scope).list().await?;

        for slice in &slices.items {
            self.check_and_emit(scope, stored, slice).await?;
        }

        Ok(())
    }

    /// Evaluate the slice's lens against the full event log and check
    /// whether the stored event is now in the result set. If so, emit
    /// a `SliceMatched` event so the projection increments the count.
    ///
    /// TODO: This re-evaluates the full lens per event per slice (`O(n^2)`).
    /// Optimize by caching inner-lens entity refs at slice creation time
    /// and checking `derive_refs(event) ∩ cached_refs ≠ ∅` per event.
    async fn check_and_emit(
        &self,
        scope: &Scope<AtBookmark>,
        stored: &StoredEvent,
        slice: &Slice,
    ) -> Result<(), EventError> {
        let lens_source = format!("events_for({})", slice.lens_expr);

        let selection = LensService::select(scope, &self.canons, &lens_source)
            .await
            .map_err(|e| EventError::Import(e.to_string()))?;

        if selection.event_ids().contains(&stored.id) {
            let matched = SliceMatched::builder_v1()
                .slice_name(slice.name.clone())
                .matched_event_id(stored.id)
                .build()
                .into();

            let new_event = NewEvent::builder()
                .data(Events::Slice(SliceEvents::SliceMatched(matched)))
                .build();

            self.mailbox.tell(ProjectMessage::from(
                AppendProjectLog::builder()
                    .scope(scope.clone())
                    .event(new_event)
                    .build(),
            ));
        }

        Ok(())
    }
}
