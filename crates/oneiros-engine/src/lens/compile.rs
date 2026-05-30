use crate::*;

pub(crate) struct Compiler {
    registry: Registry,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CompileError {
    #[error("unknown predicate: {0}")]
    UnknownPredicate(PredicateName),
    #[error("predicate {name} cannot be compiled to a read operation")]
    UncompilablePredicate { name: PredicateName },
}

#[derive(Clone, Copy)]
enum WalkContext {
    Open,
    Names(NameKind),
}

impl Compiler {
    pub(crate) fn new(registry: Registry) -> Self {
        Self { registry }
    }

    pub(crate) fn compile(&self, ast: &Lens) -> Result<Ir, CompileError> {
        let mut ops = Vec::new();
        self.walk(ast, WalkContext::Open, &mut ops)?;
        Ok(Ir::new(ops))
    }

    fn walk(
        &self,
        node: &Lens,
        context: WalkContext,
        ops: &mut Vec<Op>,
    ) -> Result<SlotId, CompileError> {
        match node {
            Lens::Predicate(predicate) => self.compile_predicate(predicate, ops),
            Lens::Union(left, right) => {
                let left_slot = self.walk(left, context, ops)?;
                let right_slot = self.walk(right, context, ops)?;
                let slot = SlotId(ops.len());
                ops.push(Op::Union(left_slot, right_slot));
                Ok(slot)
            }
            Lens::Intersection(left, right) => {
                let left_slot = self.walk(left, context, ops)?;
                let right_slot = self.walk(right, context, ops)?;
                let slot = SlotId(ops.len());
                ops.push(Op::Intersect(left_slot, right_slot));
                Ok(slot)
            }
            Lens::Difference(left, right) => {
                let left_slot = self.walk(left, context, ops)?;
                let right_slot = self.walk(right, context, ops)?;
                let slot = SlotId(ops.len());
                ops.push(Op::Difference(left_slot, right_slot));
                Ok(slot)
            }
            Lens::Ref(reference) => {
                let slot = SlotId(ops.len());
                ops.push(Op::Const(ConstValue::Ref(reference.clone())));
                Ok(slot)
            }
            Lens::Symbol(identifier) => match context {
                WalkContext::Names(kind) => {
                    let slot = SlotId(ops.len());
                    ops.push(Op::Const(ConstValue::Name {
                        name: identifier.to_string(),
                        kind,
                    }));
                    Ok(slot)
                }
                WalkContext::Open => Err(CompileError::UncompilablePredicate {
                    name: PredicateName::new("(literal)"),
                }),
            },
            Lens::String(_) | Lens::Integer(_) => Err(CompileError::UncompilablePredicate {
                name: PredicateName::new("(literal)"),
            }),
        }
    }

    fn compile_predicate(
        &self,
        predicate: &Predicate,
        ops: &mut Vec<Op>,
    ) -> Result<SlotId, CompileError> {
        let spec = self
            .registry
            .lookup(&predicate.name)
            .ok_or_else(|| CompileError::UnknownPredicate(predicate.name.clone()))?;

        match &spec.executor {
            ExecutorHint::GraphStep(step_kind) => {
                self.compile_graph_step(predicate, spec, step_kind.clone(), ops)
            }
            ExecutorHint::SearchIndexText => {
                let text = predicate
                    .args
                    .first()
                    .map(|a| match a {
                        Lens::String(s) => s.to_string(),
                        other => format!("{other:?}"),
                    })
                    .unwrap_or_default();
                let slot = SlotId(ops.len());
                ops.push(Op::Read(Read::SearchText(text)));
                Ok(slot)
            }
            ExecutorHint::ChronicleBetween => {
                let Some(Lens::Ref(from)) = predicate.args.first() else {
                    return Err(CompileError::UncompilablePredicate {
                        name: predicate.name.clone(),
                    });
                };
                let Some(Lens::Ref(to)) = predicate.args.get(1) else {
                    return Err(CompileError::UncompilablePredicate {
                        name: predicate.name.clone(),
                    });
                };
                let slot = SlotId(ops.len());
                ops.push(Op::Read(Read::ChronicleBetween {
                    from: from.clone(),
                    to: to.clone(),
                }));
                Ok(slot)
            }
        }
    }

    fn compile_graph_step(
        &self,
        predicate: &Predicate,
        spec: &PredicateSpec,
        spec_kind: StepKind,
        ops: &mut Vec<Op>,
    ) -> Result<SlotId, CompileError> {
        let Some(sub_lens) = predicate.args.first() else {
            return Err(CompileError::UncompilablePredicate {
                name: predicate.name.clone(),
            });
        };

        let sub_context = match spec.arg_types.first() {
            Some(ArgType::LensOfNames(kind)) => WalkContext::Names(*kind),
            _ => WalkContext::Open,
        };

        let input_slot = self.walk(sub_lens, sub_context, ops)?;

        let kind = match spec_kind {
            StepKind::Within(_) => {
                let Some(Lens::Integer(n)) = predicate.args.get(1) else {
                    return Err(CompileError::UncompilablePredicate {
                        name: predicate.name.clone(),
                    });
                };
                let depth = u32::try_from(n.value().max(0)).unwrap_or(0);
                StepKind::Within(depth)
            }
            other => other,
        };

        let slot = SlotId(ops.len());
        ops.push(Op::Step {
            kind,
            input: input_slot,
        });
        Ok(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compiler() -> Compiler {
        Compiler::new(Registry::seed_default())
    }

    #[test]
    fn compiles_facet_predicate_to_step_over_name_literal() {
        let ast = Lens::predicate("texture", [Lens::symbol("observation")]);
        let ir = compiler().compile(&ast).unwrap();
        assert_eq!(ir.ops.len(), 2);
        assert!(matches!(
            &ir.ops[0],
            Op::Const(ConstValue::Name {
                kind: NameKind::Texture,
                ..
            })
        ));
        assert!(matches!(
            &ir.ops[1],
            Op::Step {
                kind: StepKind::SearchByTexture,
                ..
            }
        ));
    }

    #[test]
    fn compiles_search_text_predicate() {
        let ast = Lens::predicate("search", [Lens::string("hello world")]);
        let ir = compiler().compile(&ast).unwrap();
        assert_eq!(ir.ops.len(), 1);
        assert!(matches!(&ir.ops[0], Op::Read(Read::SearchText(_))));
    }

    #[test]
    fn compiles_union_of_names_inside_facet_predicate() {
        let ast = Lens::predicate(
            "agent",
            [Lens::union(
                Lens::symbol("governor.process"),
                Lens::symbol("thinker.process"),
            )],
        );
        let ir = compiler().compile(&ast).unwrap();
        // slot 0: Const::Name(governor.process)
        // slot 1: Const::Name(thinker.process)
        // slot 2: Union(0, 1)
        // slot 3: Step{SearchByAgent, input: slot 2}
        assert_eq!(ir.ops.len(), 4);
        assert!(matches!(
            &ir.ops[3],
            Op::Step {
                kind: StepKind::SearchByAgent,
                ..
            }
        ));
    }

    #[test]
    fn rejects_unknown_predicate() {
        let ast = Lens::predicate("nonexistent", [Lens::symbol("x")]);
        let err = compiler().compile(&ast).unwrap_err();
        assert!(matches!(err, CompileError::UnknownPredicate(_)));
    }

    #[test]
    fn rejects_bare_literal_at_top_level() {
        let ast = Lens::symbol("bare");
        let err = compiler().compile(&ast).unwrap_err();
        assert!(matches!(err, CompileError::UncompilablePredicate { .. }));
    }
}
