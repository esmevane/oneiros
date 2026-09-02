use crate::*;

pub(crate) struct Explanation {
    ir: LensIntermediateRepresentation,
}

impl Explanation {
    pub(crate) fn new(ir: LensIntermediateRepresentation) -> Self {
        Self { ir }
    }
}

impl core::fmt::Display for Explanation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, op) in self.ir.ops.iter().enumerate() {
            let slot = LensSlotId(i);
            match op {
                LensOp::Const(LensConstValue::Name { name, kind }) => {
                    writeln!(f, "{slot}: const name({}: {name:?})", kind.describe())?;
                }
                LensOp::Const(LensConstValue::Ref(reference)) => {
                    writeln!(f, "{slot}: const ref({reference})")?;
                }
                LensOp::Read(LensRead::SearchText(text)) => {
                    writeln!(f, "{slot}: read search_text({text:?})")?;
                }
                LensOp::Read(LensRead::ChronicleBetween { from, to }) => {
                    writeln!(f, "{slot}: read between({from}, {to})")?;
                }
                LensOp::Step { kind, input } => {
                    writeln!(f, "{slot}: step {kind:?}({input})")?;
                }
                LensOp::Union(left, right) => {
                    writeln!(f, "{slot}: union({left}, {right})")?;
                }
                LensOp::Intersect(left, right) => {
                    writeln!(f, "{slot}: intersect({left}, {right})")?;
                }
                LensOp::Difference(left, right) => {
                    writeln!(f, "{slot}: difference({left}, {right})")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explains_name_literal_and_step() {
        let ir = LensIntermediateRepresentation::new(vec![
            LensOp::Const(LensConstValue::Name {
                name: "observation".into(),
                kind: NameKind::Texture,
            }),
            LensOp::Step {
                kind: LensStepKind::SearchByTexture,
                input: LensSlotId(0),
            },
        ]);
        let explanation = Explanation::new(ir);
        let output = explanation.to_string();
        assert!(output.contains("$0: const name(texture: \"observation\")"));
        assert!(output.contains("$1: step SearchByTexture($0)"));
    }

    #[test]
    fn explains_intersection_pipeline() {
        let ir = LensIntermediateRepresentation::new(vec![
            LensOp::Const(LensConstValue::Name {
                name: "observation".into(),
                kind: NameKind::Texture,
            }),
            LensOp::Step {
                kind: LensStepKind::SearchByTexture,
                input: LensSlotId(0),
            },
            LensOp::Const(LensConstValue::Name {
                name: "governor.process".into(),
                kind: NameKind::Agent,
            }),
            LensOp::Step {
                kind: LensStepKind::SearchByAgent,
                input: LensSlotId(2),
            },
            LensOp::Intersect(LensSlotId(1), LensSlotId(3)),
        ]);
        let explanation = Explanation::new(ir);
        let output = explanation.to_string();
        assert!(output.contains("intersect($1, $3)"));
    }
}
