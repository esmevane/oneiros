use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct Ir {
    pub(crate) ops: Vec<Op>,
}

impl Ir {
    pub(crate) fn new(ops: Vec<Op>) -> Self {
        Self { ops }
    }

    pub(crate) fn result_slot(&self) -> SlotId {
        SlotId(self.ops.len().saturating_sub(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct SlotId(pub(crate) usize);

impl core::fmt::Display for SlotId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "${}", self.0)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Op {
    Read(Read),
    Union(SlotId, SlotId),
    Intersect(SlotId, SlotId),
    Difference(SlotId, SlotId),
}

#[derive(Debug, Clone)]
pub(crate) enum Read {
    SearchFacet { facet: FacetName, value: String },
    SearchText(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_slot_points_to_last_op() {
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
        assert_eq!(ir.result_slot(), SlotId(2));
    }

    #[test]
    fn empty_ir_result_slot_is_zero() {
        let ir = Ir::new(vec![]);
        assert_eq!(ir.result_slot(), SlotId(0));
    }
}
