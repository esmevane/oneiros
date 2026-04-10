//! Follow domain lifecycle — exercised through `FollowService` directly
//! since the bookmark-verb surface that normally drives follow creation
//! lands in Act 3. These tests cover the projection round-trip and the
//! create/advance/remove state transitions via `BookmarkEvents`.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn follow_lifecycle_via_service() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?.init_system().await?;

    let context = app.config().system();

    // Start with no follows on the system.
    let initial = FollowService::list(&context, &SearchFilters::default()).await?;
    assert_eq!(initial.len(), 0);

    // Create a follow pointing at a local bookmark reference.
    let brain = BrainName::new("test-follow-brain");
    let bookmark = BookmarkName::new("test-follow-bookmark");
    let source = FollowSource::Local(Ref::bookmark(BookmarkId::new()));

    let follow = FollowService::create(&context, brain.clone(), bookmark.clone(), source).await?;

    assert_eq!(follow.brain, brain);
    assert_eq!(follow.bookmark, bookmark);
    assert!(follow.checkpoint.is_empty());

    // Listed.
    let listed = FollowService::list(&context, &SearchFilters::default()).await?;
    assert_eq!(listed.len(), 1);

    // Retrievable by id.
    let fetched = FollowService::get(&context, follow.id).await?;
    assert_eq!(fetched.id, follow.id);

    // Retrievable by bookmark.
    let by_bookmark = FollowService::for_bookmark(&context, &brain, &bookmark)
        .await?
        .expect("should find follow for this bookmark");
    assert_eq!(by_bookmark.id, follow.id);

    // Advance the checkpoint.
    let new_checkpoint = Checkpoint {
        sequence: 7,
        cumulative_hash: ContentHash::new("someHash"),
        head: Some(EventId::new()),
        taken_at: Timestamp::now(),
    };
    FollowService::advance(&context, follow.id, new_checkpoint.clone(), 7).await?;

    // Refetched follow shows the advanced checkpoint.
    let advanced = FollowService::get(&context, follow.id).await?;
    assert_eq!(advanced.checkpoint.sequence, 7);

    // Remove the follow.
    FollowService::remove(&context, follow.id).await?;

    // It's gone.
    let empty = FollowService::list(&context, &SearchFilters::default()).await?;
    assert_eq!(empty.len(), 0);

    let missing = FollowService::for_bookmark(&context, &brain, &bookmark).await?;
    assert!(missing.is_none());

    Ok(())
}
