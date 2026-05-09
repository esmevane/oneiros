use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SwitchBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum MergeBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) source: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListBookmarks {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ShareBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
            #[arg(long)]
            pub(crate) actor_id: Option<ActorId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum FollowBookmark {
        #[derive(clap::Args)]
        V1 => {
            pub(crate) uri: String,
            #[arg(long)]
            #[builder(into)]
            pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CollectBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UnfollowBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: BookmarkName,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = BookmarkRequestType, display = "kebab-case")]
pub(crate) enum BookmarkRequest {
    CreateBookmark(CreateBookmark),
    SwitchBookmark(SwitchBookmark),
    MergeBookmark(MergeBookmark),
    ListBookmarks(ListBookmarks),
    ShareBookmark(ShareBookmark),
    FollowBookmark(FollowBookmark),
    CollectBookmark(CollectBookmark),
    UnfollowBookmark(UnfollowBookmark),
}
