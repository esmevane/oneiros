//! `BookmarkProjectionsActor` — singleton that applies project projections
//! against any bookmark's database.
//!
//! First-time-seen `(project, bookmark)` pairs are caught up by replaying
//! the project event log into the bookmark DB. Subsequent `Apply`
//! messages project a single stored event each. Idempotent
//! (delete-then-insert), so partial replay or duplicate applies are
//! safe.
//!
//! Receives the full `BookmarkMessage`; handles the projection variants
//! and no-ops the rest.

use std::collections::HashSet;

use tokio::sync::mpsc;

use crate::*;

#[derive(Clone)]
pub(crate) struct BookmarkProjectionsMailbox {
    tx: mpsc::UnboundedSender<BookmarkMessage>,
}

impl BookmarkProjectionsMailbox {
    pub(crate) fn open() -> (Self, BookmarkProjectionsInbox) {
        let (tx, rx) = mpsc::unbounded_channel();
        (Self { tx }, BookmarkProjectionsInbox { rx })
    }

    pub(crate) fn tell(&self, message: BookmarkMessage) {
        if let Err(error) = self.tx.send(message) {
            tracing::warn!(error = %error, "bookmark projections: receiver closed; message dropped");
        }
    }
}

pub(crate) struct BookmarkProjectionsInbox {
    rx: mpsc::UnboundedReceiver<BookmarkMessage>,
}

impl BookmarkProjectionsInbox {
    pub(crate) async fn recv(&mut self) -> Option<BookmarkMessage> {
        self.rx.recv().await
    }
}

pub(crate) struct BookmarkProjectionsActor {
    canons: CanonIndex,
    caught_up: HashSet<(ProjectName, BookmarkName)>,
}

impl BookmarkProjectionsActor {
    pub(crate) fn spawn(inbox: BookmarkProjectionsInbox, canons: CanonIndex) {
        tokio::spawn(
            Self {
                canons,
                caught_up: HashSet::new(),
            }
            .run(inbox),
        );
    }

    async fn run(mut self, mut inbox: BookmarkProjectionsInbox) {
        while let Some(message) = inbox.recv().await {
            match message {
                BookmarkMessage::ProjectionApply(apply) => {
                    if let Err(error) = self.handle_apply(apply).await {
                        tracing::error!(?error, "bookmark projections: apply failed");
                    }
                }
                BookmarkMessage::ProjectionReset(reset) => {
                    if let Err(error) = self.replay(&reset.scope).await {
                        tracing::error!(?error, "bookmark projections: reset failed");
                    }
                }
                _ => {}
            }
        }
    }

    async fn handle_apply(&mut self, apply: ApplyBookmarkProjection) -> Result<(), EventError> {
        let key = (
            apply.scope.project().name.clone(),
            apply.scope.bookmark().name.clone(),
        );
        if !self.caught_up.contains(&key) {
            self.replay(&apply.scope).await?;
            self.caught_up.insert(key);
        }
        let bookmark_db = BookmarkDb::open(&apply.scope).await?;
        let projections = self.projections_for(&apply.scope.project().name)?;
        projections.apply_project(&bookmark_db, &apply.stored)?;
        Ok(())
    }

    async fn replay(&self, scope: &Scope<AtBookmark>) -> Result<(), EventError> {
        let bookmark_db = BookmarkDb::open(scope).await?;
        let projections = self.projections_for(&scope.project().name)?;
        projections.migrate(&bookmark_db)?;
        let log = EventLog::attached(&bookmark_db);
        projections.replay_project(&bookmark_db, &log)?;
        Ok(())
    }

    fn projections_for(
        &self,
        project: &ProjectName,
    ) -> Result<Projections<ProjectCanon>, EventError> {
        let entry = self.canons.project_entry(project)?;
        Ok(Projections::project_with_pipeline(entry.pipeline))
    }
}
