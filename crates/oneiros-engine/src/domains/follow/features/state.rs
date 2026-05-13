use crate::*;

pub(crate) struct FollowState;

impl FollowState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Bookmark(bookmark_event) = event {
            match bookmark_event {
                BookmarkEvents::BookmarkFollowed(followed) => {
                    if let Ok(current) = followed.current() {
                        let follow = Follow::builder()
                            .id(current.id)
                            .project(current.project)
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
                        canon.follows.remove(&current.follow_id);
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

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}
