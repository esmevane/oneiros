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
    pub(crate) async fn explain(
        scope: &Scope<AtBookmark>,
        request: &ExplainLens,
    ) -> Result<LensResponse, LensError> {
        let ExplainLens::V1(req) = request;
        let parsed = Lens::parse(&req.source)?;
        let resolver = AliasResolver::new(&scope.config().lens.aliases)?;
        let lens = resolver.expand(parsed)?;
        let registry = Registry::seed_default();
        lens.validate(&registry)?;

        let names = NameResolver::fetch(scope).await?;
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

    /// Executes a lens query end-to-end: parse, alias-expand, validate,
    /// name-resolve, compile, execute via readers, and return hits.
    pub(crate) async fn query(
        scope: &Scope<AtBookmark>,
        canons: &CanonIndex,
        request: &QueryLens,
    ) -> Result<LensResponse, LensError> {
        let QueryLens::V1(req) = request;
        let parsed = Lens::parse(&req.source)?;
        let resolver = AliasResolver::new(&scope.config().lens.aliases)?;
        let lens = resolver.expand(parsed)?;
        let registry = Registry::seed_default();
        lens.validate(&registry)?;

        let names = NameResolver::fetch(scope).await?;
        lens.check_names(&registry, &names)?;

        let compiler = Compiler::new(registry);
        let ir = compiler.compile(&lens)?;

        let db = BookmarkDb::open(scope).await?;
        let host_db = HostDb::open(scope).await?;
        let search_reader = SearchIndexReader::new(&db);
        let trail_reader = TrailLensReader::new(&db);
        let connection_reader = ConnectionLensReader::new(&db);
        let chronicle_reader =
            ChronicleLensReader::new(&host_db, canons, scope.project().name.clone());
        let readers: Vec<&dyn Reader> = vec![
            &search_reader,
            &trail_reader,
            &connection_reader,
            &chronicle_reader,
        ];
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

    /// Runs the lens query pipeline and returns the raw [`Selection`]
    /// without hydrating into domain objects. Callers apply their own
    /// pagination, filtering, and repo dispatch from here.
    pub(crate) async fn select(
        scope: &Scope<AtBookmark>,
        canons: &CanonIndex,
        source: &str,
    ) -> Result<Selection, LensError> {
        let request = QueryLens::builder_v1()
            .source(source.to_string())
            .build()
            .into();
        let LensResponse::Queried(QueriedLensResponse::V1(queried)) =
            Self::query(scope, canons, &request).await?
        else {
            return Ok(Selection::new());
        };
        let mut selection = Selection::new();
        for hit in &queried.hits {
            selection.insert(hit.clone());
        }
        Ok(selection)
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
