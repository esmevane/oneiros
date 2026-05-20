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
        assert_eq!(expected, "symbol");
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
        assert_eq!(spec.arg_types, vec![ArgType::Symbol]);
    }

    #[test]
    fn leaves_have_no_result_type() {
        let registry = registry();
        for source in [
            "governor.process",
            r#""text""#,
            "ref:AAQQAZ4_p6yYe4Ou_ZRRuMpwKQ",
            "42",
        ] {
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

        let events = Lens::parse("between(ref:AAA, ref:BBB)").expect("parses");
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

        let over_events = Lens::parse("recent(between(ref:AAA, ref:BBB), 12)").expect("parses");
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
        let lens =
            Lens::parse("agent(governor.process) & between(ref:AAA, ref:BBB)").expect("parses");
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
        let lens =
            Lens::parse("agent(governor.process) | between(ref:AAA, ref:BBB)").expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        let LensValidationError::ResultTypeMismatch { operator, .. } = error else {
            panic!("expected ResultTypeMismatch");
        };
        assert_eq!(operator, "|");
    }

    #[test]
    fn rejects_mismatch_under_difference() {
        let lens =
            Lens::parse("between(ref:AAA, ref:BBB) ~ agent(governor.process)").expect("parses");
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
        let lens = Lens::parse(
            "agent(governor.process) | (texture(observation) & between(ref:AAA, ref:BBB))",
        )
        .expect("parses");
        let error = lens.validate(&registry()).expect_err("must fail");
        assert!(matches!(
            error,
            LensValidationError::ResultTypeMismatch { .. }
        ));
    }

    #[test]
    fn rejects_mismatch_through_recent_inheritance() {
        // recent inherits its first arg's type → events here, mismatched with entities side.
        let lens = Lens::parse("recent(between(ref:AAA, ref:BBB), 12) & agent(governor.process)")
            .expect("parses");
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
        for source in [
            "governor.process | between(ref:AAA, ref:BBB)",
            "governor.process & agent(governor.process)",
        ] {
            let lens = Lens::parse(source).expect("parses");
            lens.validate(&registry)
                .unwrap_or_else(|err| panic!("expected `{source}` to validate, got {err}"));
        }
    }

    #[test]
    fn accepts_recent_over_either_substrate() {
        let registry = registry();
        for source in [
            "recent(agent(governor.process), 12)",
            "recent(between(ref:AAA, ref:BBB), 12)",
        ] {
            let lens = Lens::parse(source).expect("parses");
            lens.validate(&registry)
                .unwrap_or_else(|err| panic!("expected `{source}` to validate, got {err}"));
        }
    }
}
