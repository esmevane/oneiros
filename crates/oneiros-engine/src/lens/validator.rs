use crate::*;

impl Lens {
    pub(crate) fn validate(&self, registry: &Registry) -> Result<(), LensValidationError> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => Ok(()),
            Lens::Predicate(predicate) => predicate.validate(registry),
            Lens::Union(left, right) => self.validate_set_operator("|", left, right, registry),
            Lens::Intersection(left, right) => {
                self.validate_set_operator("&", left, right, registry)
            }
            Lens::Difference(left, right) => self.validate_set_operator("~", left, right, registry),
        }
    }

    pub(crate) fn check_names(
        &self,
        registry: &Registry,
        names: &dyn NameRegistry,
    ) -> Result<(), LensValidationError> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => Ok(()),
            Lens::Predicate(predicate) => predicate.check_names(registry, names),
            Lens::Union(left, right)
            | Lens::Intersection(left, right)
            | Lens::Difference(left, right) => {
                left.check_names(registry, names)?;
                right.check_names(registry, names)
            }
        }
    }

    pub(crate) fn result_type(&self, registry: &Registry) -> Option<ResultType> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => None,
            Lens::Predicate(predicate) => predicate.resolve_type(registry),
            Lens::Union(left, right)
            | Lens::Intersection(left, right)
            | Lens::Difference(left, right) => left
                .result_type(registry)
                .or_else(|| right.result_type(registry)),
        }
    }

    pub(crate) fn describe(&self) -> &'static str {
        match self {
            Lens::Symbol(_) => "symbol",
            Lens::String(_) => "string",
            Lens::Ref(_) => "ref",
            Lens::Integer(_) => "integer",
            Lens::Predicate(_) => "predicate",
            Lens::Union(_, _) => "union",
            Lens::Intersection(_, _) => "intersection",
            Lens::Difference(_, _) => "difference",
        }
    }

    fn validate_set_operator(
        &self,
        operator: &'static str,
        left: &Lens,
        right: &Lens,
        registry: &Registry,
    ) -> Result<(), LensValidationError> {
        left.validate(registry)?;
        right.validate(registry)?;
        match (left.result_type(registry), right.result_type(registry)) {
            (Some(left_type), Some(right_type)) if left_type != right_type => {
                Err(LensValidationError::ResultTypeMismatch {
                    operator,
                    left: left_type.describe(),
                    right: right_type.describe(),
                })
            }
            _ => Ok(()),
        }
    }
}

impl Predicate {
    fn validate(&self, registry: &Registry) -> Result<(), LensValidationError> {
        let Some(spec) = registry.lookup(&self.name) else {
            return Err(LensValidationError::UnknownPredicate {
                name: self.name.clone(),
            });
        };
        if spec.arg_types.len() != self.args.len() {
            return Err(LensValidationError::ArityMismatch {
                name: self.name.clone(),
                expected: spec.arg_types.len(),
                got: self.args.len(),
            });
        }
        for (position, (arg_type, arg_lens)) in spec.arg_types.iter().zip(&self.args).enumerate() {
            if !arg_type.matches(arg_lens) {
                return Err(LensValidationError::ArgTypeMismatch {
                    predicate: self.name.clone(),
                    position,
                    expected: arg_type.describe(),
                    got: arg_lens.describe(),
                });
            }
        }
        Ok(())
    }

    fn check_names(
        &self,
        registry: &Registry,
        names: &dyn NameRegistry,
    ) -> Result<(), LensValidationError> {
        let Some(spec) = registry.lookup(&self.name) else {
            return Ok(());
        };
        for (arg_type, arg_lens) in spec.arg_types.iter().zip(&self.args) {
            match (arg_type, arg_lens) {
                (ArgType::SymbolOf(kind), Lens::Symbol(identifier)) => {
                    if !names.knows(*kind, identifier) {
                        return Err(LensValidationError::UnknownSymbol {
                            kind: kind.describe(),
                            name: identifier.clone(),
                        });
                    }
                }
                (_, nested) => nested.check_names(registry, names)?,
            }
        }
        Ok(())
    }

    fn resolve_type(&self, registry: &Registry) -> Option<ResultType> {
        let spec = registry.lookup(&self.name)?;
        let SpecResultType::Of(result_type) = spec.result_type;
        Some(result_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> Registry {
        Registry::seed_default()
    }

    #[test]
    fn validates_known_predicate_with_correct_arg() {
        let lens = Lens::predicate("agent", [Lens::symbol("governor.process")]);
        lens.validate(&registry()).expect("validates");
    }

    #[test]
    fn validates_set_operators_when_both_sides_validate() {
        let lens = Lens::union(
            Lens::predicate("agent", [Lens::symbol("a")]),
            Lens::predicate("agent", [Lens::symbol("b")]),
        );
        lens.validate(&registry()).expect("validates");
    }

    #[test]
    fn rejects_unknown_predicate() {
        let lens = Lens::predicate("nonexistent", [Lens::symbol("x")]);
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::UnknownPredicate { .. }
        ));
    }

    #[test]
    fn rejects_arity_mismatch() {
        let lens = Lens::predicate("agent", [Lens::symbol("a"), Lens::symbol("b")]);
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::ArityMismatch {
                expected: 1,
                got: 2,
                ..
            }
        ));
    }

    #[test]
    fn rejects_arg_type_mismatch_symbol_where_string_expected() {
        let lens = Lens::predicate("search", [Lens::symbol("bare")]);
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(error, LensValidationError::ArgTypeMismatch { .. }));
    }

    #[test]
    fn rejects_arg_type_mismatch_integer_where_symbol_expected() {
        let lens = Lens::predicate("agent", [Lens::integer(42)]);
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(error, LensValidationError::ArgTypeMismatch { .. }));
    }

    #[test]
    fn rejects_unknown_predicate_nested_in_set_operator() {
        let lens = Lens::union(
            Lens::predicate("agent", [Lens::symbol("x")]),
            Lens::predicate("nonexistent", [Lens::symbol("y")]),
        );
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::UnknownPredicate { .. }
        ));
    }

    #[test]
    fn leaves_have_no_result_type() {
        assert_eq!(Lens::symbol("x").result_type(&registry()), None);
        assert_eq!(Lens::string("x").result_type(&registry()), None);
        assert_eq!(Lens::integer(1).result_type(&registry()), None);
    }

    #[test]
    fn predicate_result_type_resolves_via_spec() {
        let entities = Lens::predicate("agent", [Lens::symbol("x")]);
        assert_eq!(
            entities.result_type(&registry()),
            Some(ResultType::Entities)
        );
    }

    #[test]
    fn set_operator_result_type_when_both_sides_match() {
        let lens = Lens::union(
            Lens::predicate("agent", [Lens::symbol("x")]),
            Lens::predicate("texture", [Lens::symbol("y")]),
        );
        assert_eq!(lens.result_type(&registry()), Some(ResultType::Entities));
    }

    #[test]
    fn set_operator_inherits_result_type_when_one_side_is_leaf() {
        let lens = Lens::union(
            Lens::predicate("agent", [Lens::symbol("x")]),
            Lens::symbol("bare"),
        );
        assert_eq!(lens.result_type(&registry()), Some(ResultType::Entities));
    }

    #[test]
    fn set_operator_with_both_leaves_has_no_result_type() {
        let lens = Lens::union(Lens::symbol("a"), Lens::symbol("b"));
        assert_eq!(lens.result_type(&registry()), None);
    }

    #[test]
    fn lookup_returns_spec_for_known_predicate() {
        assert!(registry().lookup(&PredicateName::new("agent")).is_some());
    }

    #[test]
    fn accepts_set_operator_when_one_side_is_polymorphic_leaf() {
        Lens::union(
            Lens::predicate("agent", [Lens::symbol("x")]),
            Lens::symbol("bare"),
        )
        .validate(&registry())
        .expect("validates");
    }

    struct TestNames {
        agents: Vec<String>,
        textures: Vec<String>,
        levels: Vec<String>,
    }

    impl TestNames {
        fn new() -> Self {
            Self {
                agents: vec!["governor.process".into()],
                textures: vec!["observation".into(), "reflection".into()],
                levels: vec!["session".into()],
            }
        }
    }

    impl NameRegistry for TestNames {
        fn knows(&self, kind: NameKind, name: &Identifier) -> bool {
            match kind {
                NameKind::Agent => self.agents.iter().any(|n| n == name.as_str()),
                NameKind::Texture => self.textures.iter().any(|n| n == name.as_str()),
                NameKind::Level => self.levels.iter().any(|n| n == name.as_str()),
            }
        }
    }

    #[test]
    fn check_names_accepts_known_agent() {
        let lens = Lens::predicate("agent", [Lens::symbol("governor.process")]);
        lens.check_names(&registry(), &TestNames::new())
            .expect("valid");
    }

    #[test]
    fn check_names_rejects_unknown_agent_with_kind_label() {
        let lens = Lens::predicate("agent", [Lens::symbol("nonexistent.agent")]);
        let error = lens
            .check_names(&registry(), &TestNames::new())
            .expect_err("must fail");
        let message = format!("{error}");
        assert!(message.contains("unknown agent"));
    }

    #[test]
    fn check_names_rejects_unknown_texture() {
        let lens = Lens::predicate("texture", [Lens::symbol("nonexistent")]);
        let error = lens
            .check_names(&registry(), &TestNames::new())
            .expect_err("must fail");
        let message = format!("{error}");
        assert!(message.contains("unknown texture"));
    }

    #[test]
    fn check_names_rejects_unknown_level() {
        let lens = Lens::predicate("level", [Lens::symbol("nonexistent")]);
        let error = lens
            .check_names(&registry(), &TestNames::new())
            .expect_err("must fail");
        let message = format!("{error}");
        assert!(message.contains("unknown level"));
    }

    #[test]
    fn check_names_walks_into_set_operators() {
        let lens = Lens::union(
            Lens::predicate("agent", [Lens::symbol("governor.process")]),
            Lens::predicate("agent", [Lens::symbol("nonexistent.agent")]),
        );
        let error = lens
            .check_names(&registry(), &TestNames::new())
            .expect_err("must fail");
        let message = format!("{error}");
        assert!(message.contains("nonexistent.agent"));
    }

    #[test]
    fn check_names_ignores_untyped_symbol_positions() {
        let lens = Lens::predicate("kind", [Lens::symbol("whatever_goes_here")]);
        lens.check_names(&registry(), &TestNames::new())
            .expect("valid — kind takes untyped Symbol, no name check");
    }

    #[test]
    fn check_names_accepts_set_operator_when_both_sides_resolve() {
        let lens = Lens::union(
            Lens::predicate("agent", [Lens::symbol("governor.process")]),
            Lens::predicate("texture", [Lens::symbol("observation")]),
        );
        lens.check_names(&registry(), &TestNames::new())
            .expect("valid");
    }

    #[test]
    fn check_names_skips_unknown_predicate_silently() {
        let lens = Lens::predicate("nonexistent", [Lens::symbol("anything")]);
        lens.check_names(&registry(), &TestNames::new())
            .expect("valid — structural validator catches unknowns");
    }
}
