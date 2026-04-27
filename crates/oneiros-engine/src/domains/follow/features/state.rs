use crate::*;

pub struct FollowState;

impl FollowState {
    pub fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Bookmark(bookmark_event) = event {
            match bookmark_event {
                BookmarkEvents::BookmarkFollowed(follow) => {
                    canon.follows.set(follow);
                }
                BookmarkEvents::BookmarkUnfollowed(unfollowed) => {
                    canon.follows.remove(unfollowed.follow_id());
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
