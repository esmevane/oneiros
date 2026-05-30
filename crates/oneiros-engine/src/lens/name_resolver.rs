use std::collections::HashSet;

use crate::*;

/// Project-backed [`NameRegistry`] — fetches the full set of registered
/// names from each vocabulary table at construction time and answers
/// resolution queries synchronously against the resulting in-memory sets.
///
/// One round-trip per kind. The vocabulary domains are small
/// (typically <20 entries each) and the snapshot is per-call, so the
/// fetch cost is negligible and the resolver stays consistent for the
/// duration of a single resolution pass.
pub(crate) struct NameResolver {
    agents: HashSet<String>,
    textures: HashSet<String>,
    levels: HashSet<String>,
}

impl NameResolver {
    pub(crate) async fn fetch(scope: &Scope<AtBookmark>) -> Result<Self, EventError> {
        let db = BookmarkDb::open(scope).await?;
        Ok(Self {
            agents: Self::names_from(&db, "agents")?,
            textures: Self::names_from(&db, "textures")?,
            levels: Self::names_from(&db, "levels")?,
        })
    }

    fn names_from(db: &BookmarkDb, table: &str) -> Result<HashSet<String>, rusqlite::Error> {
        let sql = format!("select name from {table}");
        let mut stmt = db.prepare(&sql)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        rows.collect()
    }
}

impl NameRegistry for NameResolver {
    fn knows(&self, kind: NameKind, name: &Identifier) -> bool {
        let bucket = match kind {
            NameKind::Agent => &self.agents,
            NameKind::Texture => &self.textures,
            NameKind::Level => &self.levels,
            NameKind::Kind => {
                return matches!(
                    name.as_str(),
                    "cognition"
                        | "memory"
                        | "experience"
                        | "agent"
                        | "connection"
                        | "bookmark"
                        | "tenant"
                        | "actor"
                        | "ticket"
                        | "follow"
                        | "peer"
                        | "project"
                        | "storage"
                        | "texture"
                        | "level"
                        | "sensation"
                        | "nature"
                        | "persona"
                        | "urge"
                );
            }
        };
        bucket.contains(name.as_str())
    }
}
