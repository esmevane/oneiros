use crate::*;

pub(crate) struct FollowState;

impl FollowState {
    pub(crate) fn reduce(mut canon: SystemCanon, event: &Events) -> SystemCanon {
        if let Events::Bookmark(bookmark_event) = event {
            match bookmark_event {
                BookmarkEvents::BookmarkFollowed(follow) => {
                    canon.follows.set(follow);
                }
                BookmarkEvents::BookmarkUnfollowed(unfollowed) => {
                    canon.follows.remove(unfollowed.follow_id);
                }
                _ => {}
            }
        }
        canon
    }

    pub(crate) fn reducer() -> Reducer<SystemCanon> {
        Reducer::new(Self::reduce)
    }
}
