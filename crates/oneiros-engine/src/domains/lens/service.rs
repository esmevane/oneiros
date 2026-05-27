use std::collections::HashSet;

use crate::*;

pub(crate) struct LensService;

impl LensService {
    /// Parses a lens source string and returns a [`ParsedLensResponse`]
    /// carrying the source and its round-trip [`Lens::Display`] form.
    pub(crate) fn parse(request: &ParseLens) -> Result<LensResponse, LensError> {
        let ParseLens::V1(req) = request;
        let lens = Lens::parse(&req.source)?;
        Ok(LensResponse::Parsed(
            ParsedLensResponse::builder_v1()
                .source(req.source.clone())
                .display(lens.to_string())
                .build()
                .into(),
        ))
    }

    /// Parses, validates, name-resolves, and explains a lens source string.
    ///
    /// Loads canonical names from the project's vocabulary repos (agents,
    /// textures, levels, sensations, personas, natures) and uses them to
    /// catch typo-shaped errors that pure structural validation would miss
    /// — `agent(governorr.process)` rejects even though the grammar is fine.
    pub(crate) async fn explain(
        scope: &Scope<AtBookmark>,
        request: &ExplainLens,
    ) -> Result<LensResponse, LensError> {
        let ExplainLens::V1(req) = request;
        let lens = Lens::parse(&req.source)?;
        let registry = Registry::seed_default();
        lens.validate(&registry)?;

        let names = ProjectNameRegistry::fetch(scope).await?;
        lens.check_names(&registry, &names)?;

        let compiler = Compiler::new(registry);
        let ir = compiler.compile(&lens)?;
        let explanation = Explanation::new(ir);

        Ok(LensResponse::Explained(
            ExplainedLensResponse::builder_v1()
                .source(req.source.clone())
                .display(lens.to_string())
                .plan(explanation.to_string())
                .build()
                .into(),
        ))
    }
    pub(crate) async fn query(
        scope: &Scope<AtBookmark>,
        request: &QueryLens,
    ) -> Result<LensResponse, LensError> {
        let QueryLens::V1(req) = request;
        let lens = Lens::parse(&req.source)?;
        let registry = Registry::seed_default();
        lens.validate(&registry)?;

        let names = ProjectNameRegistry::fetch(scope).await?;
        lens.check_names(&registry, &names)?;

        let compiler = Compiler::new(registry);
        let ir = compiler.compile(&lens)?;

        let db = BookmarkDb::open(scope).await?;
        let search_reader = SearchIndexReader::new(&db);
        let readers: Vec<&dyn Reader> = vec![&search_reader];
        let executor = Executor::new(&readers);
        let selection = executor.run(&ir)?;

        let hits = selection.sorted_by_timestamp_desc();
        let total = hits.len();

        Ok(LensResponse::Queried(
            QueriedLensResponse::builder_v1()
                .source(req.source.clone())
                .hits(hits)
                .total(total)
                .build()
                .into(),
        ))
    }
}

/// Project-backed [`NameRegistry`] — fetches the full set of registered
/// names from each vocabulary table at construction time and answers
/// resolution queries synchronously against the resulting in-memory sets.
///
/// One round-trip per kind, six total. The vocabulary domains are small
/// (typically <20 entries each) and the snapshot is per-explain-call, so
/// the fetch cost is negligible and the registry stays consistent for the
/// duration of a single resolution pass.
pub(crate) struct ProjectNameRegistry {
    agents: HashSet<String>,
    textures: HashSet<String>,
    levels: HashSet<String>,
}

impl ProjectNameRegistry {
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

impl NameRegistry for ProjectNameRegistry {
    fn knows(&self, kind: NameKind, name: &Identifier) -> bool {
        let bucket = match kind {
            NameKind::Agent => &self.agents,
            NameKind::Texture => &self.textures,
            NameKind::Level => &self.levels,
        };
        bucket.contains(name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_returns_round_trip_display_for_valid_lens() {
        let request = ParseLens::builder_v1()
            .source("agent(governor.process)".to_string())
            .build()
            .into();
        let LensResponse::Parsed(ParsedLensResponse::V1(parsed)) =
            LensService::parse(&request).expect("parses")
        else {
            panic!("expected parsed response");
        };
        assert_eq!(parsed.source, "agent(governor.process)");
        assert_eq!(parsed.display, "agent(governor.process)");
    }

    #[test]
    fn parse_propagates_parse_errors() {
        let request = ParseLens::builder_v1()
            .source("agent(".to_string())
            .build()
            .into();
        let error = LensService::parse(&request).expect_err("must fail");
        assert!(matches!(error, LensError::Parse(_)));
    }
}
