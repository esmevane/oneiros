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
                Op::Read(Read::SearchFacet { facet, value }) => {
                    writeln!(f, "{slot}: read search_facet({facet:?}, {value:?})")?;
                }
                Op::Read(Read::SearchText(text)) => {
                    writeln!(f, "{slot}: read search_text({text:?})")?;
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
    fn explains_single_read() {
        let ir = Ir::new(vec![Op::Read(Read::SearchFacet {
            facet: FacetName::Texture,
            value: "observation".into(),
        })]);
        let explanation = Explanation::new(ir);
        let output = explanation.to_string();
        assert!(output.contains("$0: read search_facet(Texture, \"observation\")"));
    }

    #[test]
    fn explains_intersection_pipeline() {
        let ir = Ir::new(vec![
            Op::Read(Read::SearchFacet {
                facet: FacetName::Texture,
                value: "observation".into(),
            }),
            Op::Read(Read::SearchFacet {
                facet: FacetName::Agent,
                value: "governor.process".into(),
            }),
            Op::Intersect(SlotId(0), SlotId(1)),
        ]);
        let explanation = Explanation::new(ir);
        let output = explanation.to_string();
        assert!(output.contains("$0: read search_facet(Texture, \"observation\")"));
        assert!(output.contains("$1: read search_facet(Agent, \"governor.process\")"));
        assert!(output.contains("$2: intersect($0, $1)"));
    }
}
