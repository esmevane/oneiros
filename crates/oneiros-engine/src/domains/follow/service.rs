use crate::*;

pub(crate) struct FollowService;

impl FollowService {
    /// Create a Follow record and emit `BookmarkFollowed`. Used by
    /// `BookmarkService::follow` in Act 3; exposed here for direct
    /// service-layer construction and testing.
    pub(crate) async fn create(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        project: ProjectName,
        bookmark: BookmarkName,
        source: FollowSource,
    ) -> Result<Follow, FollowError> {
        let follow = Follow::builder()
            .project(project)
            .bookmark(bookmark)
            .source(source)
            .build();

        let event = BookmarkFollowed::builder_v1()
            .id(follow.id)
            .project(follow.project.clone())
            .bookmark(follow.bookmark.clone())
            .source(follow.source.clone())
            .checkpoint(follow.checkpoint.clone())
            .created_at(follow.created_at)
            .build();

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkFollowed(
                event.into(),
            )))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(follow)
    }

    pub(crate) async fn get(scope: &Scope<AtHost>, id: FollowId) -> Result<Follow, FollowError> {
        FollowRepo::new(scope)
            .fetch(id)
            .await?
            .ok_or(FollowError::NotFound(id))
    }

    pub(crate) async fn list(
        scope: &Scope<AtHost>,
        filters: &SearchFilters,
    ) -> Result<Listed<Follow>, FollowError> {
        Ok(FollowRepo::new(scope).list(filters).await?)
    }

    /// Find the active Follow for a given project/bookmark pair. Returns
    /// `None` when the bookmark isn't currently following anything.
    /// Used by `BookmarkService::collect` in Act 3.
    pub(crate) async fn for_bookmark(
        scope: &Scope<AtHost>,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<Option<Follow>, FollowError> {
        Ok(FollowRepo::new(scope)
            .for_bookmark(project, bookmark)
            .await?)
    }

    /// Advance the checkpoint on a follow after a successful collect.
    /// Emits `BookmarkCollected`. Used by `BookmarkService::collect`.
    pub(crate) async fn advance(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        follow_id: FollowId,
        checkpoint: Checkpoint,
        events_received: u64,
    ) -> Result<(), FollowError> {
        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkCollected(
                BookmarkCollected::builder_v1()
                    .follow_id(follow_id)
                    .checkpoint(checkpoint)
                    .events_received(events_received)
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(())
    }

    /// Remove a follow. Emits `BookmarkUnfollowed`. Used by
    /// `BookmarkService::unfollow`.
    pub(crate) async fn remove(
        scope: &Scope<AtHost>,
        mailbox: &Mailbox,
        id: FollowId,
    ) -> Result<(), FollowError> {
        let existing = FollowRepo::new(scope)
            .get(id)
            .await?
            .ok_or(FollowError::NotFound(id))?;

        let new_event = NewEvent::builder()
            .data(Events::Bookmark(BookmarkEvents::BookmarkUnfollowed(
                BookmarkUnfollowed::builder_v1()
                    .follow_id(existing.id)
                    .project(existing.project)
                    .bookmark(existing.bookmark)
                    .build()
                    .into(),
            )))
            .build();

        mailbox.tell(HostMessage::from(
            AppendHostLog::builder()
                .scope(scope.clone())
                .event(new_event)
                .build(),
        ));

        Ok(())
    }
}
