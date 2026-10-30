//! Persistence — a scope appending to its log, and knowing it landed.
//!
//! Every scope tier that writes events writes to exactly one log: a
//! bookmark scope to its project's log, a host scope to the host log.
//! [`HasLog`] captures that, so services don't spell out the append
//! message themselves:
//!
//! ```rust,ignore
//! mailbox.tell(scope.append(new_event.clone()));
//! scope.ensure_persisted(&new_event).await?;
//! ```
//!
//! `tell` stays fire-and-forget; the mailbox routes the message and knows
//! nothing about events. Waiting is a separate, optional step: events
//! carry their id from birth, so recognising one in the log is a lookup,
//! and every append wakes the waiters to look again — no polling
//! interval.

use crate::*;

/// A scope that writes to an event log.
///
/// `HasHost` is the base capability every tier has (config and the
/// connection pool) — not the host log. Which log a scope writes to is
/// decided by its impl below.
pub(crate) trait HasLog: HasHost {
    /// The message that appends `event` to this scope's log.
    fn append(&self, event: NewEvent) -> Message;

    /// Check out a handle to this scope's log.
    async fn log_db(&self) -> Result<DbHandle, DbError>;

    /// Wait until `event` is in this scope's log, up to
    /// `database.persist_timeout`.
    ///
    /// `Ok` means the event is durable. `NotPersisted` means it hadn't
    /// shown up in time — most likely the append failed, though a log
    /// stalled past the deadline looks the same from here.
    async fn ensure_persisted(&self, event: &NewEvent) -> Result<(), EventError> {
        let databases = self.databases();
        let deadline = tokio::time::Instant::now() + self.config().database.persist_timeout;

        loop {
            let appended = databases.appended();
            tokio::pin!(appended);
            appended.as_mut().enable();

            let log = self.log_db().await?;
            let events = EventLog::new(&log);
            // The log actor creates the table on first append; until
            // then there's nothing to find.
            events.init()?;
            let found = events.contains(event.id)?;
            drop(log);

            if found {
                return Ok(());
            }

            if tokio::time::timeout_at(deadline, appended).await.is_err() {
                return Err(EventError::NotPersisted(event.id));
            }
        }
    }
}

/// A bookmark scope writes to its project's log.
impl HasLog for Scope<AtBookmark> {
    fn append(&self, event: NewEvent) -> Message {
        ProjectMessage::from(
            AppendProjectLog::builder()
                .scope(self.clone())
                .event(event)
                .build(),
        )
        .into()
    }

    async fn log_db(&self) -> Result<DbHandle, DbError> {
        self.project_log().await
    }
}

/// A host scope writes to the host log.
impl HasLog for Scope<AtHost> {
    fn append(&self, event: NewEvent) -> Message {
        HostMessage::from(
            AppendHostLog::builder()
                .scope(self.clone())
                .event(event)
                .build(),
        )
        .into()
    }

    async fn log_db(&self) -> Result<DbHandle, DbError> {
        self.host_db().await
    }
}
