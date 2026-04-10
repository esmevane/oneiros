//! Bookmark share workflow — mint a distribution ticket for a bookmark
//! and verify the returned URI round-trips through the OneirosUri parser
//! with a Peer variant whose target points at the shared bookmark.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn share_bookmark_returns_roundtrippable_uri() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();
    let brain = BrainName::new("test");

    // Find the actor created during init_system — it's the only one.
    let actor_id = match client.actor().list(&ListActors::builder().build()).await? {
        ActorResponse::Listed(listed) => {
            assert!(
                !listed.items.is_empty(),
                "init_system should have seeded an actor"
            );
            listed.items[0].data.id
        }
        other => panic!("expected Listed actors, got {other:?}"),
    };

    // Share the main bookmark.
    let request = ShareBookmark::builder()
        .name(BookmarkName::main())
        .actor_id(actor_id)
        .build();

    let share_result = match client.bookmark().share(&brain, &request).await? {
        BookmarkResponse::Shared(result) => result,
        other => panic!("expected Shared, got {other:?}"),
    };

    // Ticket should carry the bookmark target inside its link.
    let target = &share_result.ticket.link.target;
    assert!(
        matches!(target, Ref::V0(Resource::Bookmark(_))),
        "ticket target should be a bookmark ref, got {target:?}"
    );
    assert_eq!(share_result.ticket.granted_by, actor_id);
    assert_eq!(share_result.ticket.actor_id, actor_id);

    // URI should parse as OneirosUri::Peer and contain a PeerLink whose
    // link matches the ticket's link (same target + token).
    let parsed: OneirosUri = share_result.uri.parse()?;
    let peer_link = match parsed {
        OneirosUri::Peer(pl) => pl,
        other => panic!("expected OneirosUri::Peer, got {other:?}"),
    };

    assert_eq!(peer_link.link.target, *target);
    assert_eq!(
        peer_link.link.token.as_str(),
        share_result.ticket.link.token.as_str()
    );

    Ok(())
}
