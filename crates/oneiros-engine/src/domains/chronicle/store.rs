use rusqlite::params;

use crate::*;

/// Content-addressed object store for chronicle HAMT nodes.
///
/// Nodes are stored by the BLAKE3/SHA256 hash of their serialized bytes.
/// Structural sharing means identical subtrees are stored only once.
pub struct ChronicleStore<'a> {
    db: &'a rusqlite::Connection,
}

impl<'a> ChronicleStore<'a> {
    pub fn new(db: &'a rusqlite::Connection) -> Self {
        Self { db }
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.db.execute_batch(
            "CREATE TABLE IF NOT EXISTS chronicle_objects (
                hash TEXT PRIMARY KEY,
                data BLOB NOT NULL
            )",
        )?;
        Ok(())
    }

    /// Store a ledger node, returning its content hash.
    /// If the hash already exists, this is a no-op (content-addressing = dedup).
    pub fn put(&self, node: &LedgerNode) -> ContentHash {
        let bytes = serde_json::to_vec(node).unwrap_or_default();
        let hash = ContentHash::compute(&bytes);

        let _ = self.db.execute(
            "INSERT OR IGNORE INTO chronicle_objects (hash, data) VALUES (?1, ?2)",
            params![hash.to_string(), bytes],
        );

        hash
    }

    /// Retrieve a ledger node by its content hash.
    pub fn get(&self, hash: &ContentHash) -> Option<LedgerNode> {
        let mut stmt = self
            .db
            .prepare_cached("SELECT data FROM chronicle_objects WHERE hash = ?1")
            .ok()?;

        let bytes: Vec<u8> = stmt
            .query_row(params![hash.to_string()], |row| row.get(0))
            .ok()?;

        serde_json::from_slice(&bytes).ok()
    }

    /// Create a resolve closure for use with Ledger operations.
    pub fn resolver(&self) -> impl Fn(&ContentHash) -> Option<LedgerNode> + '_ {
        move |hash| self.get(hash)
    }

    /// Create a store closure for use with Ledger operations.
    pub fn writer(&self) -> impl Fn(&LedgerNode) -> ContentHash + '_ {
        move |node| self.put(node)
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.db.execute_batch("DELETE FROM chronicle_objects")?;
        Ok(())
    }
}
