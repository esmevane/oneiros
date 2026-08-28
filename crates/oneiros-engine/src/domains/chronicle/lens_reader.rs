use rusqlite::params;
use std::future::Future;
use std::pin::Pin;

use crate::*;

pub(crate) struct ChronicleLensReader {
    host_db: DbHandle,
    canons: CanonIndex,
    project: ProjectName,
}

impl ChronicleLensReader {
    pub(crate) fn new(host_db: DbHandle, canons: CanonIndex, project: ProjectName) -> Self {
        Self {
            host_db,
            canons,
            project,
        }
    }

    fn resolve_bookmark_name(&self, reference: &RefToken) -> Result<BookmarkName, ReaderError> {
        let Ref::V0(Resource::Bookmark(bookmark_id)) = reference.inner() else {
            return Err(ReaderError::Internal(format!(
                "between() expected bookmark ref, got {reference}"
            )));
        };

        let name: String = self
            .host_db
            .query_row(
                "SELECT name FROM bookmarks WHERE id = ?1 AND project = ?2",
                params![bookmark_id.to_string(), self.project.to_string()],
                |row| row.get(0),
            )
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        Ok(BookmarkName::new(name))
    }

    pub(crate) async fn read_between(
        &self,
        from: &RefToken,
        to: &RefToken,
    ) -> Result<Selection, ReaderError> {
        let from_name = self.resolve_bookmark_name(from)?;
        let to_name = self.resolve_bookmark_name(to)?;

        let from_chronicle = self
            .canons
            .bookmark_chronicle(&self.project, &from_name)
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let to_chronicle = self
            .canons
            .bookmark_chronicle(&self.project, &to_name)
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        let from_root = from_chronicle
            .root()
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let to_root = to_chronicle
            .root()
            .map_err(|e| ReaderError::Internal(e.to_string()))?;

        let store = ChronicleStore::new(&self.host_db);
        let resolver = store.resolver();
        let changes = Ledger::diff(from_root.as_ref(), to_root.as_ref(), &|hash| {
            std::future::ready(resolver(hash))
        })
        .await;

        let mut selection = Selection::new();
        for change in changes {
            if let LedgerChange::Added(event_id) = change {
                selection.insert(Hit::Event(EventHit {
                    event_id,
                    timestamp: Timestamp::now(),
                    relevance: Relevance::Unknown,
                }));
            }
        }
        Ok(selection)
    }
}

impl LensReader for ChronicleLensReader {
    fn read<'a>(
        &'a self,
        read: &'a LensRead,
    ) -> Pin<Box<dyn Future<Output = Option<Result<Selection, ReaderError>>> + Send + 'a>> {
        match read {
            LensRead::ChronicleBetween { from, to } => {
                Box::pin(async move { Some(self.read_between(from, to).await) })
            }
            LensRead::SearchText(_) => Box::pin(async { None }),
        }
    }
}
