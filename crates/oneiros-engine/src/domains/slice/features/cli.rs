use clap::Subcommand;

use crate::*;

/// CLI subcommands for the slice domain.
#[derive(Debug, Subcommand)]
pub(crate) enum SliceCommands {
    Create(CreateSlice),
    List(ListSlices),
    Delete(DeleteSlice),
    Diff(DiffSlice),
    Bookmark(BookmarkSlice),
}

impl SliceCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, SliceError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Create(create) => create.execute_request(&client).await?,
            Self::List(list) => list.execute_request(&client).await?,
            Self::Delete(delete) => delete.execute_request(&client).await?,
            Self::Diff(diff) => diff.execute_request(&client).await?,
            Self::Bookmark(bookmark) => bookmark.execute_request(&client).await?,
        };

        let response: SliceResponse = serde_json::from_slice(&bytes)?;
        Ok(SliceView::new(response).render().map(Into::into))
    }
}
