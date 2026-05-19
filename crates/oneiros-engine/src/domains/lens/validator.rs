use crate::*;

impl Lens {
    pub(crate) fn validate(&self, registry: &Registry) -> Result<(), LensValidationError> {
        match self {
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => Ok(()),
            Lens::Predicate(predicate) => validate_predicate(predicate, registry),
            Lens::Union(left, right)
            | Lens::Intersection(left, right)
            | Lens::Difference(left, right) => {
                left.validate(registry)?;
                right.validate(registry)
            }
        }
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
        let lens =
            Lens::parse("texture(observation) & agent(governor.process)").expect("parses");
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
        assert!(matches!(error, LensValidationError::UnknownPredicate { .. }));
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
        let LensValidationError::ArgTypeMismatch {
            expected, got, ..
        } = error
        else {
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
}
