use crate::*;

impl Lens {
    pub(crate) fn validate(&self, registry: &Registry) -> Result<(), LensValidationError> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => Ok(()),
            Lens::Predicate(predicate) => validate_predicate(predicate, registry),
            Lens::Union(left, right) => validate_set_operator("|", left, right, registry),
            Lens::Intersection(left, right) => validate_set_operator("&", left, right, registry),
            Lens::Difference(left, right) => validate_set_operator("~", left, right, registry),
        }
    }

    /// Cross-checks every `SymbolOf(kind)` argument against the supplied
    /// [`NameRegistry`]. Assumes the lens has already passed
    /// [`Lens::validate`]; walks predicates and verifies each typed-symbol
    /// position resolves to a registered name of its declared kind.
    ///
    /// Unknown predicates are silently skipped here (the structural validator
    /// catches them); a `SymbolOf` arg whose lens isn't actually a symbol is
    /// also skipped (likewise structural). The only error this returns is
    /// [`LensValidationError::UnknownSymbol`].
    pub(crate) fn check_names(
        &self,
        registry: &Registry,
        names: &dyn NameRegistry,
    ) -> Result<(), LensValidationError> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => Ok(()),
            Lens::Predicate(predicate) => check_predicate_names(predicate, registry, names),
            Lens::Union(left, right)
            | Lens::Intersection(left, right)
            | Lens::Difference(left, right) => {
                left.check_names(registry, names)?;
                right.check_names(registry, names)
            }
        }
    }

    /// Resolves the lens's result type against the registry, recursing through
    /// predicates and set operators.
    ///
    /// Leaves (symbols, strings, refs, integers) return `None` — they're
    /// polymorphic until symbol resolution (c2) binds them to a typed kind.
    /// Predicates resolve via their spec; `InheritsFromArg(i)` recurses into
    /// the named argument. Set operators return the common type when both
    /// sides agree, or whichever side is typed when the other defers.
    /// Returns `None` for an unknown predicate (the validator catches that).
    pub(crate) fn result_type(&self, registry: &Registry) -> Option<ResultType> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => None,
            Lens::Predicate(predicate) => resolve_predicate_type(predicate, registry),
            Lens::Union(left, right)
            | Lens::Intersection(left, right)
            | Lens::Difference(left, right) => left
                .result_type(registry)
                .or_else(|| right.result_type(registry)),
        }
    }
}

fn check_predicate_names(
    predicate: &Predicate,
    registry: &Registry,
    names: &dyn NameRegistry,
) -> Result<(), LensValidationError> {
    let Some(spec) = registry.lookup(&predicate.name) else {
        return Ok(());
    };
    for (arg_type, arg_lens) in spec.arg_types.iter().zip(&predicate.args) {
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

fn resolve_predicate_type(predicate: &Predicate, registry: &Registry) -> Option<ResultType> {
    let spec = registry.lookup(&predicate.name)?;
    match spec.result_type {
        SpecResultType::Of(result_type) => Some(result_type),
        SpecResultType::InheritsFromArg(index) => predicate.args.get(index)?.result_type(registry),
    }
}

fn validate_set_operator(
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

fn validate_predicate(
    predicate: &Predicate,
    registry: &Registry,
) -> Result<(), LensValidationError> {
    let Some(spec) = registry.lookup(&predicate.name) else {
        return Err(LensValidationError::UnknownPredicate {
            name: predicate.name.clone(),
        });
    };
    if spec.arg_types.len() != predicate.args.len() {
        return Err(LensValidationError::ArityMismatch {
            name: predicate.name.clone(),
            expected: spec.arg_types.len(),
            got: predicate.args.len(),
        });
    }
    for (position, (arg_type, arg_lens)) in spec.arg_types.iter().zip(&predicate.args).enumerate() {
        if !arg_type.matches(arg_lens) {
            return Err(LensValidationError::ArgTypeMismatch {
                predicate: predicate.name.clone(),
                position,
                expected: arg_type.describe(),
                got: describe_lens(arg_lens),
            });
        }
        if matches!(arg_type, ArgType::Lens) {
            arg_lens.validate(registry)?;
        }
    }
    Ok(())
}

fn describe_lens(lens: &Lens) -> &'static str {
    match lens {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn registry() -> Registry {
        Registry::seed_default()
    }

    /// Produce a fresh, valid `ref:<base64>` string for test fixtures.
    /// Used wherever a test needs *some* ref literal in source — the
    /// specific identity doesn't matter, only that the parser accepts it.
    fn fake_ref() -> String {
        crate::RefToken::new(crate::Ref::cognition(crate::CognitionId::new())).to_string()
    }

    #[test]
    fn validates_known_predicate_with_correct_arg() {
        let lens = Lens::parse("agent(governor.process)").expect("parses");
        lens.validate(&registry()).expect("validates");
    }

    #[test]
    fn validates_nested_predicate_argument() {
        // `from` takes a Lens, level(working) is one — exercises nested validation.
        let lens = Lens::parse("from(level(working))").expect("parses");
        lens.validate(&registry()).expect("validates");
    }

    #[test]
    fn validates_set_operators_when_both_sides_validate() {
        let lens = Lens::parse("texture(observation) & agent(governor.process)").expect("parses");
        lens.validate(&registry()).expect("validates");
    }

    #[test]
    fn validates_integer_argument_position() {
        let lens = Lens::parse("recent(agent(governor.process), 12)").expect("parses");
        lens.validate(&registry()).expect("validates");
    }

    #[test]
    fn rejects_unknown_predicate() {
        let lens = Lens::parse("unknown(x)").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::UnknownPredicate { .. }
        ));
    }

    #[test]
    fn rejects_unknown_predicate_nested_in_set_operator() {
        let lens = Lens::parse("agent(a) & bogus(b)").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::UnknownPredicate { name } = error else {
            panic!("expected UnknownPredicate, got something else");
        };
        assert_eq!(name.as_str(), "bogus");
    }

    #[test]
    fn rejects_unknown_predicate_nested_in_argument() {
        let lens = Lens::parse("from(bogus(x))").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::UnknownPredicate { name } = error else {
            panic!("expected UnknownPredicate");
        };
        assert_eq!(name.as_str(), "bogus");
    }

    #[test]
    fn rejects_arity_mismatch() {
        let lens = Lens::parse("agent()").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::ArityMismatch {
                expected: 1,
                got: 0,
                ..
            }
        ));
    }

    #[test]
    fn rejects_arg_type_mismatch_integer_where_symbol_expected() {
        let lens = Lens::parse("agent(12)").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::ArgTypeMismatch { expected, got, .. } = error else {
            panic!("expected ArgTypeMismatch");
        };
        assert_eq!(expected, "agent symbol");
        assert_eq!(got, "integer");
    }

    #[test]
    fn rejects_arg_type_mismatch_symbol_where_string_expected() {
        let lens = Lens::parse("search(naming)").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(error, LensValidationError::ArgTypeMismatch { .. }));
    }

    #[test]
    fn rejects_arg_type_mismatch_integer_where_lens_expected_in_recent() {
        // recent expects (Lens, Integer); flip the args.
        let lens = Lens::parse("recent(12, agent(governor.process))").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(error, LensValidationError::ArgTypeMismatch { .. }));
    }

    #[test]
    fn lookup_returns_spec_for_known_predicate() {
        let registry = registry();
        let agent_name = PredicateName::new("agent");
        let spec = registry.lookup(&agent_name).expect("agent is registered");
        assert_eq!(spec.arg_types, vec![ArgType::SymbolOf(NameKind::Agent)]);
    }

    #[test]
    fn leaves_have_no_result_type() {
        let registry = registry();
        let ref_source = fake_ref();
        let sources = [
            "governor.process",
            r#""text""#,
            ref_source.as_str(),
            "42",
        ];
        for source in sources {
            let lens = Lens::parse(source).expect("parses");
            assert_eq!(
                lens.result_type(&registry),
                None,
                "leaf `{source}` should have no result type yet",
            );
        }
    }

    #[test]
    fn predicate_result_type_resolves_via_spec() {
        let registry = registry();
        let entities = Lens::parse("agent(governor.process)").expect("parses");
        assert_eq!(entities.result_type(&registry), Some(ResultType::Entities));

        let source = format!("between({}, {})", fake_ref(), fake_ref());
        let events = Lens::parse(&source).expect("parses");
        assert_eq!(events.result_type(&registry), Some(ResultType::Events));
    }

    #[test]
    fn recent_inherits_result_type_from_first_arg() {
        let registry = registry();
        let over_entities = Lens::parse("recent(agent(governor.process), 12)").expect("parses");
        assert_eq!(
            over_entities.result_type(&registry),
            Some(ResultType::Entities),
        );

        let source = format!("recent(between({}, {}), 12)", fake_ref(), fake_ref());
        let over_events = Lens::parse(&source).expect("parses");
        assert_eq!(over_events.result_type(&registry), Some(ResultType::Events),);
    }

    #[test]
    fn set_operator_result_type_when_both_sides_match() {
        let registry = registry();
        let lens = Lens::parse("agent(governor.process) & texture(observation)").expect("parses");
        assert_eq!(lens.result_type(&registry), Some(ResultType::Entities));
    }

    #[test]
    fn set_operator_inherits_result_type_when_one_side_is_leaf() {
        let registry = registry();
        let lens = Lens::parse("governor.process | agent(governor.process)").expect("parses");
        assert_eq!(lens.result_type(&registry), Some(ResultType::Entities));
    }

    #[test]
    fn set_operator_with_both_leaves_has_no_result_type() {
        let registry = registry();
        let lens = Lens::parse("governor.process | other.agent").expect("parses");
        assert_eq!(lens.result_type(&registry), None);
    }

    #[test]
    fn rejects_set_operator_with_mismatched_result_types() {
        let source = format!(
            "agent(governor.process) & between({}, {})",
            fake_ref(),
            fake_ref()
        );
        let lens = Lens::parse(&source).expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::ResultTypeMismatch {
            operator,
            left,
            right,
        } = error
        else {
            panic!("expected ResultTypeMismatch");
        };
        assert_eq!(operator, "&");
        assert_eq!(left, "entities");
        assert_eq!(right, "events");
    }

    #[test]
    fn rejects_mismatch_inside_union() {
        let source = format!(
            "agent(governor.process) | between({}, {})",
            fake_ref(),
            fake_ref()
        );
        let lens = Lens::parse(&source).expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::ResultTypeMismatch { operator, .. } = error else {
            panic!("expected ResultTypeMismatch");
        };
        assert_eq!(operator, "|");
    }

    #[test]
    fn rejects_mismatch_under_difference() {
        let source = format!(
            "between({}, {}) ~ agent(governor.process)",
            fake_ref(),
            fake_ref()
        );
        let lens = Lens::parse(&source).expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::ResultTypeMismatch {
            operator,
            left,
            right,
        } = error
        else {
            panic!("expected ResultTypeMismatch");
        };
        assert_eq!(operator, "~");
        assert_eq!(left, "events");
        assert_eq!(right, "entities");
    }

    #[test]
    fn rejects_mismatch_nested_inside_a_set_operator() {
        let source = format!(
            "agent(governor.process) | (texture(observation) & between({}, {}))",
            fake_ref(),
            fake_ref()
        );
        let lens = Lens::parse(&source).expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::ResultTypeMismatch { .. }
        ));
    }

    #[test]
    fn rejects_mismatch_through_recent_inheritance() {
        // recent inherits its first arg's type → events here, mismatched with entities side.
        let source = format!(
            "recent(between({}, {}), 12) & agent(governor.process)",
            fake_ref(),
            fake_ref()
        );
        let lens = Lens::parse(&source).expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::ResultTypeMismatch { left, right, .. } = error else {
            panic!("expected ResultTypeMismatch");
        };
        assert_eq!(left, "events");
        assert_eq!(right, "entities");
    }

    #[test]
    fn accepts_set_operator_when_one_side_is_polymorphic_leaf() {
        // Bare symbol defers typing until c2; validator stays permissive.
        let registry = registry();
        let with_event_side = format!("governor.process | between({}, {})", fake_ref(), fake_ref());
        let sources: Vec<&str> = vec![
            with_event_side.as_str(),
            "governor.process & agent(governor.process)",
        ];
        for source in sources {
            let lens = Lens::parse(source).expect("parses");
            lens.validate(&registry)
                .unwrap_or_else(|err| panic!("expected `{source}` to validate, got {err}"));
        }
    }

    #[test]
    fn accepts_recent_over_either_substrate() {
        let registry = registry();
        let over_events = format!("recent(between({}, {}), 12)", fake_ref(), fake_ref());
        let sources: Vec<&str> = vec![
            "recent(agent(governor.process), 12)",
            over_events.as_str(),
        ];
        for source in sources {
            let lens = Lens::parse(source).expect("parses");
            lens.validate(&registry)
                .unwrap_or_else(|err| panic!("expected `{source}` to validate, got {err}"));
        }
    }

    /// Test-only [`NameRegistry`] backed by a per-kind set of identifiers.
    /// Used to exercise [`Lens::check_names`] without standing up project
    /// repos.
    struct FakeNameRegistry {
        agents: std::collections::HashSet<String>,
        textures: std::collections::HashSet<String>,
        levels: std::collections::HashSet<String>,
        sensations: std::collections::HashSet<String>,
        personas: std::collections::HashSet<String>,
        natures: std::collections::HashSet<String>,
    }

    impl FakeNameRegistry {
        fn empty() -> Self {
            Self {
                agents: Default::default(),
                textures: Default::default(),
                levels: Default::default(),
                sensations: Default::default(),
                personas: Default::default(),
                natures: Default::default(),
            }
        }

        fn with_agent(mut self, name: &str) -> Self {
            self.agents.insert(name.to_string());
            self
        }

        fn with_texture(mut self, name: &str) -> Self {
            self.textures.insert(name.to_string());
            self
        }

        fn with_level(mut self, name: &str) -> Self {
            self.levels.insert(name.to_string());
            self
        }
    }

    impl NameRegistry for FakeNameRegistry {
        fn knows(&self, kind: NameKind, name: &Identifier) -> bool {
            let bucket = match kind {
                NameKind::Agent => &self.agents,
                NameKind::Texture => &self.textures,
                NameKind::Level => &self.levels,
                NameKind::Sensation => &self.sensations,
                NameKind::Persona => &self.personas,
                NameKind::Nature => &self.natures,
            };
            bucket.contains(name.as_str())
        }
    }

    #[test]
    fn check_names_accepts_known_agent() {
        let lens = Lens::parse("agent(governor.process)").expect("parses");
        let names = FakeNameRegistry::empty().with_agent("governor.process");
        lens.check_names(&registry(), &names).expect("name resolves");
    }

    #[test]
    fn check_names_rejects_unknown_agent_with_kind_label() {
        let lens = Lens::parse("agent(governorr.process)").expect("parses");
        let names = FakeNameRegistry::empty().with_agent("governor.process");
        let error = lens
            .check_names(&registry(), &names)
            .expect_err("typo must reject");
        let LensValidationError::UnknownSymbol { kind, name } = error else {
            panic!("expected UnknownSymbol");
        };
        assert_eq!(kind, "agent");
        assert_eq!(name.as_str(), "governorr.process");
    }

    #[test]
    fn check_names_rejects_unknown_texture() {
        let lens = Lens::parse("texture(observasion)").expect("parses");
        let names = FakeNameRegistry::empty().with_texture("observation");
        let error = lens
            .check_names(&registry(), &names)
            .expect_err("typo must reject");
        let LensValidationError::UnknownSymbol { kind, .. } = error else {
            panic!("expected UnknownSymbol");
        };
        assert_eq!(kind, "texture");
    }

    #[test]
    fn check_names_rejects_unknown_level() {
        let lens = Lens::parse("level(woking)").expect("parses");
        let names = FakeNameRegistry::empty().with_level("working");
        let error = lens
            .check_names(&registry(), &names)
            .expect_err("typo must reject");
        let LensValidationError::UnknownSymbol { kind, .. } = error else {
            panic!("expected UnknownSymbol");
        };
        assert_eq!(kind, "level");
    }

    #[test]
    fn check_names_walks_into_set_operators() {
        // Typo on the right side of an intersection.
        let lens = Lens::parse("agent(governor.process) & texture(observasion)").expect("parses");
        let names = FakeNameRegistry::empty()
            .with_agent("governor.process")
            .with_texture("observation");
        let error = lens
            .check_names(&registry(), &names)
            .expect_err("nested typo must reject");
        let LensValidationError::UnknownSymbol { kind, name } = error else {
            panic!("expected UnknownSymbol");
        };
        assert_eq!(kind, "texture");
        assert_eq!(name.as_str(), "observasion");
    }

    #[test]
    fn check_names_walks_into_nested_predicate_arguments() {
        // Typo inside a Lens-shaped arg: `from(level(woking))`.
        let lens = Lens::parse("from(level(woking))").expect("parses");
        let names = FakeNameRegistry::empty().with_level("working");
        let error = lens
            .check_names(&registry(), &names)
            .expect_err("nested typo must reject");
        let LensValidationError::UnknownSymbol { kind, name } = error else {
            panic!("expected UnknownSymbol");
        };
        assert_eq!(kind, "level");
        assert_eq!(name.as_str(), "woking");
    }

    #[test]
    fn check_names_ignores_untyped_symbol_positions() {
        // `kind(X)` stays as ArgType::Symbol (entity kinds aren't yet a
        // NameRegistry concern). A bare-symbol arg should not be checked.
        let lens = Lens::parse("kind(anything)").expect("parses");
        let names = FakeNameRegistry::empty();
        lens.check_names(&registry(), &names)
            .expect("untyped symbol is not name-checked");
    }

    #[test]
    fn check_names_accepts_set_operator_when_both_sides_resolve() {
        let lens =
            Lens::parse("agent(governor.process) | texture(observation)").expect("parses");
        let names = FakeNameRegistry::empty()
            .with_agent("governor.process")
            .with_texture("observation");
        lens.check_names(&registry(), &names)
            .expect("both sides resolve");
    }

    #[test]
    fn check_names_skips_unknown_predicate_silently() {
        // Structural validator catches unknown predicates; check_names
        // assumes pre-validated and quietly skips.
        let lens = Lens::parse("unknown(anything)").expect("parses");
        let names = FakeNameRegistry::empty();
        lens.check_names(&registry(), &names)
            .expect("unknown predicates are the structural validator's job");
    }
}
