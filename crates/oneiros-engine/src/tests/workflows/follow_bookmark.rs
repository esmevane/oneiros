//! Bookmark follow / unfollow workflow — parse a URI, create a Follow
//! record, then remove it. Exercises the glue between BookmarkService,
//! FollowService, and PeerService.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn follow_then_unfollow_local_ref() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();
    let brain = BrainName::new("test");

    // Construct a Ref URI pointing at an arbitrary bookmark id. The
    // follow service creates the Follow record without verifying the
    // target exists — existence is checked by collect, not follow.
    let target_ref = Ref::bookmark(BookmarkId::new());
    let uri = RefToken::new(target_ref.clone()).to_string();

    let follow_request = FollowBookmark::builder()
        .uri(uri)
        .name(BookmarkName::new("mirror"))
        .build();

    let follow = match client.bookmark().follow(&brain, &follow_request).await? {
        BookmarkResponse::Followed(follow) => follow,
        other => panic!("expected Followed, got {other:?}"),
    };

    assert_eq!(follow.bookmark, BookmarkName::new("mirror"));
    assert_eq!(follow.brain, brain);
    assert!(
        matches!(follow.source, FollowSource::Local(_)),
        "ref URI should produce a Local follow"
    );

    // The Follow record is retrievable via FollowService (service layer).
    let context = app.config().system();
    let found = FollowService::for_bookmark(&context, &brain, &BookmarkName::new("mirror"))
        .await?
        .expect("follow should be in the projection");
    assert_eq!(found.id, follow.id);

    // Unfollow removes the record.
    let unfollow_request = UnfollowBookmark::builder()
        .name(BookmarkName::new("mirror"))
        .build();
    match client
        .bookmark()
        .unfollow(&brain, &unfollow_request)
        .await?
    {
        BookmarkResponse::Unfollowed(u) => {
            assert_eq!(u.follow_id, follow.id);
            assert_eq!(u.bookmark, BookmarkName::new("mirror"));
        }
        other => panic!("expected Unfollowed, got {other:?}"),
    }

    // Gone from the projection.
    let after = FollowService::for_bookmark(&context, &brain, &BookmarkName::new("mirror")).await?;
    assert!(after.is_none());

    Ok(())
}

#[tokio::test]
async fn follow_peer_uri_ensures_peer_is_known() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();
    let brain = BrainName::new("test");

    // Construct a full oneiros:// URI with a random peer and bookmark.
    let secret = iroh::SecretKey::generate(&mut rand::rng());
    let endpoint_id = secret.public();
    let host = PeerAddress::new(iroh::EndpointAddr::new(endpoint_id));
    let link = Link::new(
        Ref::bookmark(BookmarkId::new()),
        Token::from("test-share-token"),
    );
    let peer_link = PeerLink::new(host, link);
    let uri = OneirosUri::Peer(peer_link).to_string();

    // Before follow, no peers are known.
    match client.peer().list(&ListPeers::default()).await? {
        PeerResponse::Listed(listed) => assert_eq!(listed.len(), 0),
        other => panic!("expected Listed, got {other:?}"),
    }

    // Follow the URI — should auto-create the peer.
    let request = FollowBookmark::builder()
        .uri(uri)
        .name(BookmarkName::new("alice"))
        .build();

    let follow = match client.bookmark().follow(&brain, &request).await? {
        BookmarkResponse::Followed(follow) => follow,
        other => panic!("expected Followed, got {other:?}"),
    };

    assert!(
        matches!(follow.source, FollowSource::Peer(_)),
        "oneiros:// URI should produce a Peer follow"
    );

    // The peer should now be in the known-peers table.
    match client.peer().list(&ListPeers::default()).await? {
        PeerResponse::Listed(listed) => {
            assert_eq!(listed.len(), 1, "peer should have been auto-added");
            let expected_key = PeerKey::from_bytes(*endpoint_id.as_bytes());
            assert_eq!(listed.items[0].data.key, expected_key);
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    Ok(())
}
