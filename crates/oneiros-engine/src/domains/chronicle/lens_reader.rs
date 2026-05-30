use rusqlite::params;

use crate::*;

pub(crate) struct ChronicleLensReader<'a> {
    host_db: &'a HostDb,
    canons: &'a CanonIndex,
    project: ProjectName,
}

impl<'a> ChronicleLensReader<'a> {
    pub(crate) fn new(host_db: &'a HostDb, canons: &'a CanonIndex, project: ProjectName) -> Self {
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

        let mut stmt = self
            .host_db
            .prepare("SELECT name FROM bookmarks WHERE id = ?1 AND project = ?2")
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        let name: String = stmt
            .query_row(
                params![bookmark_id.to_string(), self.project.to_string()],
                |row| row.get(0),
            )
            .map_err(|e| ReaderError::Internal(e.to_string()))?;
        Ok(BookmarkName::new(name))
    }

    fn read_between(&self, from: &RefToken, to: &RefToken) -> Result<Selection, ReaderError> {
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

        let store = ChronicleStore::new(self.host_db);
        let resolver = store.resolver();
        let changes = Ledger::diff(from_root.as_ref(), to_root.as_ref(), &resolver);

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

impl Reader for ChronicleLensReader<'_> {
    fn read(&self, read: &Read) -> Option<Result<Selection, ReaderError>> {
        match read {
            Read::ChronicleBetween { from, to } => Some(self.read_between(from, to)),
            _ => None,
        }
    }
}
