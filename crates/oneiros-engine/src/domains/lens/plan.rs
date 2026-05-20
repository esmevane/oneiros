use crate::*;

/// A parallel tree to [`Lens`] that annotates each predicate with its
/// [`Executor`] hint and resolved [`ResultType`]. Leaf values (symbols,
/// strings, refs, integers) appear as bare leaves. Set operators preserve
/// their compositional shape and carry the type they evaluated to.
///
/// `result_type` is `None` only when every node beneath is a leaf — once
/// symbol resolution (c2) lands, those bind to typed kinds and this becomes
/// fully typed.
///
/// Produced by [`Lens::explain`]. Inspect programmatically or render via
/// [`core::fmt::Display`] for a tree view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExplainPlan {
    Leaf(Lens),
    Predicate {
        name: PredicateName,
        executor: Executor,
        result_type: Option<ResultType>,
        args: Vec<ExplainPlan>,
    },
    Union {
        left: Box<ExplainPlan>,
        right: Box<ExplainPlan>,
        result_type: Option<ResultType>,
    },
    Intersection {
        left: Box<ExplainPlan>,
        right: Box<ExplainPlan>,
        result_type: Option<ResultType>,
    },
    Difference {
        left: Box<ExplainPlan>,
        right: Box<ExplainPlan>,
        result_type: Option<ResultType>,
    },
}

impl Lens {
    pub(crate) fn explain(
        &self,
        registry: &Registry,
    ) -> Result<ExplainPlan, LensValidationError> {
        self.validate(registry)?;
        Ok(build_plan(self, registry))
    }
}

fn build_plan(lens: &Lens, registry: &Registry) -> ExplainPlan {
    match lens {
        Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => {
            ExplainPlan::Leaf(lens.clone())
        }
        Lens::Predicate(predicate) => {
            let spec = registry
                .lookup(&predicate.name)
                .expect("validated predicate must be in registry");
            let args = predicate
                .args
                .iter()
                .map(|arg| build_plan(arg, registry))
                .collect();
            ExplainPlan::Predicate {
                name: predicate.name.clone(),
                executor: spec.executor.clone(),
                result_type: lens.result_type(registry),
                args,
            }
        }
        Lens::Union(left, right) => ExplainPlan::Union {
            left: Box::new(build_plan(left, registry)),
            right: Box::new(build_plan(right, registry)),
            result_type: lens.result_type(registry),
        },
        Lens::Intersection(left, right) => ExplainPlan::Intersection {
            left: Box::new(build_plan(left, registry)),
            right: Box::new(build_plan(right, registry)),
            result_type: lens.result_type(registry),
        },
        Lens::Difference(left, right) => ExplainPlan::Difference {
            left: Box::new(build_plan(left, registry)),
            right: Box::new(build_plan(right, registry)),
            result_type: lens.result_type(registry),
        },
    }
}

impl core::fmt::Display for ExplainPlan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        render(self, f, 0)
    }
}

fn render(plan: &ExplainPlan, f: &mut core::fmt::Formatter<'_>, depth: usize) -> core::fmt::Result {
    let indent = "  ".repeat(depth);
    match plan {
        ExplainPlan::Leaf(lens) => writeln!(f, "{indent}{lens}"),
        ExplainPlan::Predicate {
            name,
            executor,
            result_type,
            args,
        } => {
            writeln!(
                f,
                "{indent}{name}(...) [{}{}]",
                describe_executor(executor),
                describe_result_type(*result_type),
            )?;
            for arg in args {
                render(arg, f, depth + 1)?;
            }
            Ok(())
        }
        ExplainPlan::Union {
            left,
            right,
            result_type,
        } => {
            writeln!(
                f,
                "{indent}union{}",
                describe_operator_type(*result_type)
            )?;
            render(left, f, depth + 1)?;
            render(right, f, depth + 1)
        }
        ExplainPlan::Intersection {
            left,
            right,
            result_type,
        } => {
            writeln!(
                f,
                "{indent}intersection{}",
                describe_operator_type(*result_type)
            )?;
            render(left, f, depth + 1)?;
            render(right, f, depth + 1)
        }
        ExplainPlan::Difference {
            left,
            right,
            result_type,
        } => {
            writeln!(
                f,
                "{indent}difference{}",
                describe_operator_type(*result_type)
            )?;
            render(left, f, depth + 1)?;
            render(right, f, depth + 1)
        }
    }
}

fn describe_result_type(result_type: Option<ResultType>) -> String {
    match result_type {
        Some(rt) => format!(" → {}", rt.describe()),
        None => String::new(),
    }
}

fn describe_operator_type(result_type: Option<ResultType>) -> String {
    match result_type {
        Some(rt) => format!(" [{}]", rt.describe()),
        None => String::new(),
    }
}

fn describe_executor(executor: &Executor) -> String {
    match executor {
        Executor::SearchIndexText => "search-index:text".into(),
        Executor::SearchIndexFacet(facet) => {
            format!("search-index:{}", describe_facet(*facet))
        }
        Executor::ChronicleWalk => "chronicle-walk".into(),
        Executor::GraphWalk => "graph-walk".into(),
        Executor::Recency => "recency".into(),
        Executor::ConnectionTable => "connections".into(),
        Executor::Unspecified => "unspecified".into(),
    }
}

fn describe_facet(facet: SearchFacet) -> &'static str {
    match facet {
        SearchFacet::Kind => "kind",
        SearchFacet::Agent => "agent",
        SearchFacet::Texture => "texture",
        SearchFacet::Level => "level",
        SearchFacet::Sensation => "sensation",
        SearchFacet::Persona => "persona",
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
    fn explain_validates_first_and_propagates_errors() {
        let lens = Lens::parse("unknown(x)").expect("parses");
        let error = lens.explain(&registry()).expect_err("must fail validation");
        assert!(matches!(error, LensValidationError::UnknownPredicate { .. }));
    }

    #[test]
    fn explain_annotates_facet_predicate_as_search_index() {
        let lens = Lens::parse("agent(governor.process)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Predicate {
            name,
            executor,
            result_type,
            args,
        } = plan
        else {
            panic!("expected predicate plan");
        };
        assert_eq!(name.as_str(), "agent");
        assert_eq!(executor, Executor::SearchIndexFacet(SearchFacet::Agent));
        assert_eq!(result_type, Some(ResultType::Entities));
        assert_eq!(args.len(), 1);
        assert!(matches!(args[0], ExplainPlan::Leaf(Lens::Symbol(_))));
    }

    #[test]
    fn explain_annotates_search_with_search_index_text() {
        let lens = Lens::parse(r#"search("naming")"#).expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Predicate { executor, .. } = plan else {
            panic!("expected predicate plan");
        };
        assert_eq!(executor, Executor::SearchIndexText);
    }

    #[test]
    fn explain_annotates_connection_navigation_predicates() {
        for predicate_text in [
            "from(governor.process)",
            "to(governor.process)",
            "descendants(governor.process)",
            "ancestors(governor.process)",
            "component(governor.process)",
        ] {
            let lens = Lens::parse(predicate_text).expect("parses");
            let plan = lens.explain(&registry()).expect("explains");
            let ExplainPlan::Predicate { executor, .. } = plan else {
                panic!("expected predicate plan for {predicate_text}");
            };
            assert_eq!(
                executor,
                Executor::ConnectionTable,
                "wrong executor for {predicate_text}"
            );
        }
    }

    #[test]
    fn explain_annotates_within_with_depth_argument() {
        let lens = Lens::parse("within(governor.process, 3)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Predicate {
            name,
            executor,
            args,
            ..
        } = plan
        else {
            panic!("expected predicate plan");
        };
        assert_eq!(name.as_str(), "within");
        assert_eq!(executor, Executor::ConnectionTable);
        assert_eq!(args.len(), 2);
        assert!(matches!(args[0], ExplainPlan::Leaf(Lens::Symbol(_))));
        assert!(matches!(args[1], ExplainPlan::Leaf(Lens::Integer(_))));
    }

    #[test]
    fn explain_annotates_recent_with_recency() {
        let lens = Lens::parse("recent(agent(governor.process), 12)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Predicate { executor, args, .. } = plan else {
            panic!("expected predicate plan");
        };
        assert_eq!(executor, Executor::Recency);
        // First arg is itself a planned predicate (the inner agent(...)).
        assert!(matches!(
            args[0],
            ExplainPlan::Predicate { ref executor, .. }
                if *executor == Executor::SearchIndexFacet(SearchFacet::Agent)
        ));
        // Second arg is an integer leaf.
        assert!(matches!(args[1], ExplainPlan::Leaf(Lens::Integer(_))));
    }

    #[test]
    fn explain_preserves_set_operator_structure() {
        let lens =
            Lens::parse("texture(observation) & agent(governor.process)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Intersection { left, right, .. } = plan else {
            panic!("expected intersection");
        };
        assert!(matches!(
            *left,
            ExplainPlan::Predicate { ref executor, .. }
                if *executor == Executor::SearchIndexFacet(SearchFacet::Texture)
        ));
        assert!(matches!(
            *right,
            ExplainPlan::Predicate { ref executor, .. }
                if *executor == Executor::SearchIndexFacet(SearchFacet::Agent)
        ));
    }

    #[test]
    fn explain_renders_as_indented_tree() {
        let lens =
            Lens::parse("texture(observation) & agent(governor.process)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let rendered = plan.to_string();
        assert_eq!(
            rendered,
            "intersection [entities]\n  \
             texture(...) [search-index:texture → entities]\n    \
             observation\n  \
             agent(...) [search-index:agent → entities]\n    \
             governor.process\n"
        );
    }

    #[test]
    fn explain_renders_recent_with_integer_leaf() {
        let lens = Lens::parse("recent(agent(governor.process), 12)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let rendered = plan.to_string();
        assert_eq!(
            rendered,
            "recent(...) [recency → entities]\n  \
             agent(...) [search-index:agent → entities]\n    \
             governor.process\n  \
             12\n"
        );
    }

    #[test]
    fn explain_renders_connection_navigation_with_connections_executor() {
        let lens = Lens::parse("from(governor.process)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let rendered = plan.to_string();
        assert_eq!(
            rendered,
            "from(...) [connections → entities]\n  \
             governor.process\n"
        );
    }

    #[test]
    fn explain_renders_within_with_depth_argument() {
        let lens = Lens::parse("within(governor.process, 3)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let rendered = plan.to_string();
        assert_eq!(
            rendered,
            "within(...) [connections → entities]\n  \
             governor.process\n  \
             3\n"
        );
    }

    #[test]
    fn explain_carries_result_type_on_between_predicate() {
        let lens = Lens::parse("between(ref:AAA, ref:BBB)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Predicate {
            executor,
            result_type,
            ..
        } = plan
        else {
            panic!("expected predicate plan");
        };
        assert_eq!(executor, Executor::ChronicleWalk);
        assert_eq!(result_type, Some(ResultType::Events));
    }

    #[test]
    fn explain_carries_result_type_on_set_operator() {
        let lens =
            Lens::parse("texture(observation) & agent(governor.process)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Intersection { result_type, .. } = plan else {
            panic!("expected intersection");
        };
        assert_eq!(result_type, Some(ResultType::Entities));
    }

    #[test]
    fn explain_carries_result_type_through_recent_inheritance() {
        let lens = Lens::parse("recent(between(ref:AAA, ref:BBB), 12)").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let ExplainPlan::Predicate {
            executor,
            result_type,
            ..
        } = plan
        else {
            panic!("expected predicate plan");
        };
        assert_eq!(executor, Executor::Recency);
        assert_eq!(result_type, Some(ResultType::Events));
    }

    #[test]
    fn explain_renders_polymorphic_set_operator_without_type_annotation() {
        // Bare-symbol leaves on both sides → no resolved type until c2.
        let lens = Lens::parse("governor.process | other.agent").expect("parses");
        let plan = lens.explain(&registry()).expect("explains");
        let rendered = plan.to_string();
        assert_eq!(
            rendered,
            "union\n  \
             governor.process\n  \
             other.agent\n"
        );
    }
}
