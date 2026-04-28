use crate::*;

pub struct FollowState;

impl FollowState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Bookmark(bookmark_event) = event {
            match bookmark_event {
                BookmarkEvents::BookmarkFollowed(followed) => {
                    if let Ok(current) = followed.current() {
                        let follow = Follow::builder()
                            .id(current.id)
                            .brain(current.brain)
                            .bookmark(current.bookmark)
                            .source(current.source)
                            .checkpoint(current.checkpoint)
                            .created_at(current.created_at)
                            .build();
                        canon.follows.set(&follow);
                    }
                }
                BookmarkEvents::BookmarkUnfollowed(unfollowed) => {
                    if let Ok(current) = unfollowed.current() {
                        canon.follows.remove(current.follow_id);
                    }
                }
                BookmarkEvents::BookmarkCreated(_)
                | BookmarkEvents::BookmarkForked(_)
                | BookmarkEvents::BookmarkSwitched(_)
                | BookmarkEvents::BookmarkMerged(_)
                | BookmarkEvents::BookmarkShared(_)
                | BookmarkEvents::BookmarkCollected(_) => {}
            }
        }
        canon
    }

    pub fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
