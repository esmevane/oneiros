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

    /// Parses, validates, and explains a lens source string against the
    /// default seeded registry. Returns the source, the parsed display
    /// form, and the planned tree rendered as text.
    pub(crate) fn explain(request: &ExplainLens) -> Result<LensResponse, LensError> {
        let ExplainLens::V1(req) = request;
        let lens = Lens::parse(&req.source)?;
        let registry = Registry::seed_default();
        let plan = lens.explain(&registry)?;
        Ok(LensResponse::Explained(
            ExplainedLensResponse::builder_v1()
                .source(req.source.clone())
                .display(lens.to_string())
                .plan(plan.to_string())
                .build()
                .into(),
        ))
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

    #[test]
    fn explain_returns_planned_tree() {
        let request = ExplainLens::builder_v1()
            .source("agent(governor.process)".to_string())
            .build()
            .into();
        let LensResponse::Explained(ExplainedLensResponse::V1(explained)) =
            LensService::explain(&request).expect("explains")
        else {
            panic!("expected explained response");
        };
        assert_eq!(explained.source, "agent(governor.process)");
        assert!(explained.plan.contains("search-index:agent"));
        assert!(explained.plan.contains("entities"));
    }

    #[test]
    fn explain_propagates_validation_errors() {
        let request = ExplainLens::builder_v1()
            .source("unknown(x)".to_string())
            .build()
            .into();
        let error = LensService::explain(&request).expect_err("must fail");
        assert!(matches!(error, LensError::Validate(_)));
    }

    #[test]
    fn explain_propagates_result_type_mismatch() {
        let request = ExplainLens::builder_v1()
            .source("agent(governor.process) & between(ref:AAA, ref:BBB)".to_string())
            .build()
            .into();
        let error = LensService::explain(&request).expect_err("must fail");
        let LensError::Validate(LensValidationError::ResultTypeMismatch { .. }) = error else {
            panic!("expected ResultTypeMismatch under LensError::Validate");
        };
    }
}
