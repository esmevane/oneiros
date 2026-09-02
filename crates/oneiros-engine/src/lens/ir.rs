use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct LensIntermediateRepresentation {
    pub(crate) ops: Vec<LensOp>,
}

impl LensIntermediateRepresentation {
    pub(crate) fn new(ops: Vec<LensOp>) -> Self {
        Self { ops }
    }

    pub(crate) fn result_slot(&self) -> LensSlotId {
        LensSlotId(self.ops.len().saturating_sub(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LensSlotId(pub(crate) usize);

impl core::fmt::Display for LensSlotId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "${}", self.0)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LensOp {
    /// A compile-time constant — resolved without consulting any reader.
    Const(LensConstValue),
    /// A substrate query dispatched to [`Reader::read`].
    Read(LensRead),
    /// A transformation over a prior slot dispatched to [`Reader::step`].
    Step {
        kind: LensStepKind,
        input: LensSlotId,
    },
    Union(LensSlotId, LensSlotId),
    Intersect(LensSlotId, LensSlotId),
    Difference(LensSlotId, LensSlotId),
}

/// Values fixed at compile time that the executor populates directly.
#[derive(Debug, Clone)]
pub(crate) enum LensConstValue {
    Name { name: String, kind: NameKind },
    Ref(RefToken),
}

#[derive(Debug, Clone)]
pub(crate) enum LensRead {
    SearchText(String),
    ChronicleBetween { from: RefToken, to: RefToken },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LensStepKind {
    ConnectedFrom,
    ConnectedTo,
    Descendants,
    Ancestors,
    Within(u32),
    Component,
    EventsFor,
    RefsFrom,
    SearchByAgent,
    SearchByTexture,
    SearchByLevel,
    SearchByKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_slot_points_to_last_op() {
        let ir = LensIntermediateRepresentation::new(vec![
            LensOp::Const(LensConstValue::Name {
                name: "observation".into(),
                kind: NameKind::Texture,
            }),
            LensOp::Const(LensConstValue::Name {
                name: "governor.process".into(),
                kind: NameKind::Agent,
            }),
            LensOp::Intersect(LensSlotId(0), LensSlotId(1)),
        ]);
        assert_eq!(ir.result_slot(), LensSlotId(2));
    }

    #[test]
    fn empty_ir_result_slot_is_zero() {
        let ir = LensIntermediateRepresentation::new(vec![]);
        assert_eq!(ir.result_slot(), LensSlotId(0));
    }
}
