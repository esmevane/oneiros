use crate::*;

pub struct BookmarkService;

impl BookmarkService {
    pub async fn create(
        context: &SystemContext,
        canons: &CanonIndex,
        brain: &BrainName,
        CreateBookmark { name }: &CreateBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let from = canons.active_bookmark(brain)?;
        canons.fork_brain(brain, name)?;

        let forked = BookmarkForked {
            brain: brain.clone(),
            name: name.clone(),
            from,
        };

        context
            .emit(BookmarkEvents::BookmarkForked(forked.clone()))
            .await?;

        Ok(BookmarkResponse::Forked(forked))
    }

    pub async fn switch(
        context: &SystemContext,
        canons: &CanonIndex,
        config: &Config,
        brain: &BrainName,
        SwitchBookmark { name }: &SwitchBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let old_chronicle = canons.chronicle(brain)?;
        canons.switch_brain(brain, name)?;
        let new_chronicle = canons.chronicle(brain)?;

        Self::rebuild_projections(config, brain, &old_chronicle, &new_chronicle)?;

        let switched = BookmarkSwitched {
            brain: brain.clone(),
            name: name.clone(),
        };

        context
            .emit(BookmarkEvents::BookmarkSwitched(switched.clone()))
            .await?;

        Ok(BookmarkResponse::Switched(switched))
    }

    pub async fn merge(
        context: &SystemContext,
        canons: &CanonIndex,
        config: &Config,
        brain: &BrainName,
        MergeBookmark { source }: &MergeBookmark,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let target = canons.active_bookmark(brain)?;
        canons.merge_brain(brain, source, &target)?;

        Self::rebuild_all_projections(config, brain)?;

        let merged = BookmarkMerged {
            brain: brain.clone(),
            source: source.clone(),
            target,
        };

        context
            .emit(BookmarkEvents::BookmarkMerged(merged.clone()))
            .await?;

        Ok(BookmarkResponse::Merged(merged))
    }

    pub async fn list(
        context: &SystemContext,
        brain: &BrainName,
        ListBookmarks { filters }: &ListBookmarks,
    ) -> Result<BookmarkResponse, BookmarkError> {
        let listed = BookmarkRepo::new(context).list(brain, filters).await?;
        Ok(BookmarkResponse::Bookmarks(listed))
    }

    /// Rebuild all SQLite projections from the event log.
    /// Used after merge when both branches' events should be reflected.
    fn rebuild_all_projections(config: &Config, brain: &BrainName) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();

        let db = brain_config.brain_db()?;
        let projections = Projections::<BrainCanon>::project();
        projections.migrate(&db)?;
        projections.reset(&db)?;

        let all_events = EventLog::new(&db).load_all()?;
        for event in &all_events {
            projections.apply_frames(&db, event)?;
        }

        Ok(())
    }

    /// Rebuild SQLite projections after a bookmark switch.
    ///
    /// Diffs the old and new chronicles to find which events changed,
    /// then resets and replays the new bookmark's events through projections.
    fn rebuild_projections(
        config: &Config,
        brain: &BrainName,
        old_chronicle: &Chronicle,
        new_chronicle: &Chronicle,
    ) -> Result<(), BookmarkError> {
        let mut brain_config = config.clone();
        brain_config.brain = brain.clone();

        let db = brain_config.brain_db()?;
        let chronicle_store = ChronicleStore::new(&db);
        chronicle_store.migrate()?;

        let changes = old_chronicle.diff(new_chronicle, &chronicle_store.resolver())?;

        if !changes.is_empty() {
            let projections = Projections::<BrainCanon>::project();
            let event_log = EventLog::new(&db);
            let all_events = event_log.load_all()?;

            let new_root = new_chronicle.root()?;
            let new_event_ids: std::collections::HashSet<String> =
                Ledger::collect_all_ids(new_root.as_ref(), &chronicle_store.resolver());

            projections.migrate(&db)?;
            projections.reset(&db)?;

            for event in &all_events {
                if new_event_ids.contains(&event.id.to_string()) {
                    projections.apply_frames(&db, event)?;
                }
            }
        }

        Ok(())
    }
}
