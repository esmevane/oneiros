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

impl Compiler {
    pub(crate) fn new(registry: Registry) -> Self {
        Self { registry }
    }

    pub(crate) fn compile(&self, ast: &Lens) -> Result<Ir, CompileError> {
        let mut ops = Vec::new();
        self.walk(ast, &mut ops)?;
        Ok(Ir::new(ops))
    }

    fn walk(&self, node: &Lens, ops: &mut Vec<Op>) -> Result<SlotId, CompileError> {
        match node {
            Lens::Predicate(predicate) => self.compile_predicate(predicate, ops),
            Lens::Union(left, right) => {
                let left_slot = self.walk(left, ops)?;
                let right_slot = self.walk(right, ops)?;
                let slot = SlotId(ops.len());
                ops.push(Op::Union(left_slot, right_slot));
                Ok(slot)
            }
            Lens::Intersection(left, right) => {
                let left_slot = self.walk(left, ops)?;
                let right_slot = self.walk(right, ops)?;
                let slot = SlotId(ops.len());
                ops.push(Op::Intersect(left_slot, right_slot));
                Ok(slot)
            }
            Lens::Difference(left, right) => {
                let left_slot = self.walk(left, ops)?;
                let right_slot = self.walk(right, ops)?;
                let slot = SlotId(ops.len());
                ops.push(Op::Difference(left_slot, right_slot));
                Ok(slot)
            }
            Lens::Symbol(_) | Lens::String(_) | Lens::Ref(_) | Lens::Integer(_) => {
                Err(CompileError::UncompilablePredicate {
                    name: PredicateName::new("(literal)"),
                })
            }
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

        let read = match &spec.executor {
            ExecutorHint::SearchIndexFacet(facet) => {
                let value = predicate
                    .args
                    .first()
                    .map(|a| match a {
                        Lens::Symbol(s) => s.to_string(),
                        Lens::String(s) => s.to_string(),
                        other => format!("{other:?}"),
                    })
                    .unwrap_or_default();
                let facet_name = match facet {
                    SearchFacet::Kind => FacetName::Kind,
                    SearchFacet::Agent => FacetName::Agent,
                    SearchFacet::Texture => FacetName::Texture,
                    SearchFacet::Level => FacetName::Level,
                };
                Read::SearchFacet {
                    facet: facet_name,
                    value,
                }
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
                Read::SearchText(text)
            }
        };

        let slot = SlotId(ops.len());
        ops.push(Op::Read(read));
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
    fn compiles_single_facet_predicate() {
        let ast = Lens::predicate("texture", [Lens::symbol("observation")]);
        let ir = compiler().compile(&ast).unwrap();
        assert_eq!(ir.ops.len(), 1);
        assert!(matches!(
            &ir.ops[0],
            Op::Read(Read::SearchFacet {
                facet: FacetName::Texture,
                ..
            })
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
    fn compiles_intersection_of_two_predicates() {
        let ast = Lens::intersection(
            Lens::predicate("texture", [Lens::symbol("observation")]),
            Lens::predicate("agent", [Lens::symbol("governor.process")]),
        );
        let ir = compiler().compile(&ast).unwrap();
        assert_eq!(ir.ops.len(), 3);
        assert!(matches!(&ir.ops[0], Op::Read(_)));
        assert!(matches!(&ir.ops[1], Op::Read(_)));
        assert!(matches!(&ir.ops[2], Op::Intersect(SlotId(0), SlotId(1))));
    }

    #[test]
    fn compiles_union() {
        let ast = Lens::union(
            Lens::predicate("texture", [Lens::symbol("observation")]),
            Lens::predicate("texture", [Lens::symbol("reflection")]),
        );
        let ir = compiler().compile(&ast).unwrap();
        assert_eq!(ir.ops.len(), 3);
        assert!(matches!(&ir.ops[2], Op::Union(SlotId(0), SlotId(1))));
    }

    #[test]
    fn rejects_unknown_predicate() {
        let ast = Lens::predicate("nonexistent", [Lens::symbol("x")]);
        let err = compiler().compile(&ast).unwrap_err();
        assert!(matches!(err, CompileError::UnknownPredicate(_)));
    }

    #[test]
    fn rejects_bare_literal() {
        let ast = Lens::symbol("bare");
        let err = compiler().compile(&ast).unwrap_err();
        assert!(matches!(err, CompileError::UncompilablePredicate { .. }));
    }
}
