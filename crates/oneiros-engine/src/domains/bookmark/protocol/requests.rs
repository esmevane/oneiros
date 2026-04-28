use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SwitchBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum MergeBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub source: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListBookmarks {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ShareBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: BookmarkName,
            #[arg(long)]
            pub actor_id: Option<ActorId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum FollowBookmark {
        #[derive(clap::Args)]
        V1 => {
            pub uri: String,
            #[arg(long)]
            #[builder(into)]
            pub name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum CollectBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: BookmarkName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UnfollowBookmark {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: BookmarkName,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = BookmarkRequestType, display = "kebab-case")]
pub enum BookmarkRequest {
    CreateBookmark(CreateBookmark),
    SwitchBookmark(SwitchBookmark),
    MergeBookmark(MergeBookmark),
    ListBookmarks(ListBookmarks),
    ShareBookmark(ShareBookmark),
    FollowBookmark(FollowBookmark),
    CollectBookmark(CollectBookmark),
    UnfollowBookmark(UnfollowBookmark),
}
