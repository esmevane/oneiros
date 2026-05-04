use crate::*;

pub struct FollowService;

impl FollowService {
    /// Create a Follow record and emit `BookmarkFollowed`. Used by
    /// `BookmarkService::follow` in Act 3; exposed here for direct
    /// service-layer construction and testing.
    pub async fn create(
        context: &HostLog,
        brain: BrainName,
        bookmark: BookmarkName,
        source: FollowSource,
    ) -> Result<Follow, FollowError> {
        let follow = Follow::builder()
            .brain(brain)
            .bookmark(bookmark)
            .source(source)
            .build();

        let event = BookmarkFollowed::builder_v1()
            .id(follow.id)
            .brain(follow.brain.clone())
            .bookmark(follow.bookmark.clone())
            .source(follow.source.clone())
            .checkpoint(follow.checkpoint.clone())
            .created_at(follow.created_at)
            .build();

        context
            .emit(BookmarkEvents::BookmarkFollowed(event.into()))
            .await?;

        Ok(follow)
    }

    pub async fn get(context: &HostLog, id: FollowId) -> Result<Follow, FollowError> {
        FollowRepo::new(context.scope()?)
            .fetch(id)
            .await?
            .ok_or(FollowError::NotFound(id))
    }

    pub async fn list(
        context: &HostLog,
        filters: &SearchFilters,
    ) -> Result<Listed<Follow>, FollowError> {
        Ok(FollowRepo::new(context.scope()?).list(filters).await?)
    }

    /// Find the active Follow for a given brain/bookmark pair. Returns
    /// `None` when the bookmark isn't currently following anything.
    /// Used by `BookmarkService::collect` in Act 3.
    pub async fn for_bookmark(
        context: &HostLog,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<Option<Follow>, FollowError> {
        Ok(FollowRepo::new(context.scope()?)
            .for_bookmark(brain, bookmark)
            .await?)
    }

    /// Advance the checkpoint on a follow after a successful collect.
    /// Emits `BookmarkCollected`. Used by `BookmarkService::collect`.
    pub async fn advance(
        context: &HostLog,
        follow_id: FollowId,
        checkpoint: Checkpoint,
        events_received: u64,
    ) -> Result<(), FollowError> {
        context
            .emit(BookmarkEvents::BookmarkCollected(
                BookmarkCollected::builder_v1()
                    .follow_id(follow_id)
                    .checkpoint(checkpoint)
                    .events_received(events_received)
                    .build()
                    .into(),
            ))
            .await?;
        Ok(())
    }

    /// Remove a follow. Emits `BookmarkUnfollowed`. Used by
    /// `BookmarkService::unfollow`.
    pub async fn remove(context: &HostLog, id: FollowId) -> Result<(), FollowError> {
        let existing = FollowRepo::new(context.scope()?)
            .get(id)
            .await?
            .ok_or(FollowError::NotFound(id))?;

        context
            .emit(BookmarkEvents::BookmarkUnfollowed(
                BookmarkUnfollowed::builder_v1()
                    .follow_id(existing.id)
                    .brain(existing.brain)
                    .bookmark(existing.bookmark)
                    .build()
                    .into(),
            ))
            .await?;

        Ok(())
    }
}
