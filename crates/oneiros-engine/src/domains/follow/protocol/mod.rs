mod errors;
mod requests;
mod responses;

pub use errors::*;
pub use requests::*;
pub use responses::*;

// Follow lifecycle events live on BookmarkEvents — follows are
// bookmark-native, and having their events in the bookmark stream keeps
// the lifecycle visible where it happens. There is no `FollowEvents`.
