use crate::*;

pub(crate) struct Explanation {
    ir: Ir,
}

impl Explanation {
    pub(crate) fn new(ir: Ir) -> Self {
        Self { ir }
    }
}

impl core::fmt::Display for Explanation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, op) in self.ir.ops.iter().enumerate() {
            let slot = SlotId(i);
            match op {
                Op::Const(ConstValue::Name { name, kind }) => {
                    writeln!(f, "{slot}: const name({}: {name:?})", kind.describe())?;
                }
                Op::Const(ConstValue::Ref(reference)) => {
                    writeln!(f, "{slot}: const ref({reference})")?;
                }
                Op::Read(Read::SearchText(text)) => {
                    writeln!(f, "{slot}: read search_text({text:?})")?;
                }
                Op::Read(Read::ChronicleBetween { from, to }) => {
                    writeln!(f, "{slot}: read between({from}, {to})")?;
                }
                Op::Step { kind, input } => {
                    writeln!(f, "{slot}: step {kind:?}({input})")?;
                }
                Op::Union(left, right) => {
                    writeln!(f, "{slot}: union({left}, {right})")?;
                }
                Op::Intersect(left, right) => {
                    writeln!(f, "{slot}: intersect({left}, {right})")?;
                }
                Op::Difference(left, right) => {
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
        let ir = Ir::new(vec![
            Op::Const(ConstValue::Name {
                name: "observation".into(),
                kind: NameKind::Texture,
            }),
            Op::Step {
                kind: StepKind::SearchByTexture,
                input: SlotId(0),
            },
        ]);
        let explanation = Explanation::new(ir);
        let output = explanation.to_string();
        assert!(output.contains("$0: const name(texture: \"observation\")"));
        assert!(output.contains("$1: step SearchByTexture($0)"));
    }

    #[test]
    fn explains_intersection_pipeline() {
        let ir = Ir::new(vec![
            Op::Const(ConstValue::Name {
                name: "observation".into(),
                kind: NameKind::Texture,
            }),
            Op::Step {
                kind: StepKind::SearchByTexture,
                input: SlotId(0),
            },
            Op::Const(ConstValue::Name {
                name: "governor.process".into(),
                kind: NameKind::Agent,
            }),
            Op::Step {
                kind: StepKind::SearchByAgent,
                input: SlotId(2),
            },
            Op::Intersect(SlotId(1), SlotId(3)),
        ]);
        let explanation = Explanation::new(ir);
        let output = explanation.to_string();
        assert!(output.contains("intersect($1, $3)"));
    }
}
