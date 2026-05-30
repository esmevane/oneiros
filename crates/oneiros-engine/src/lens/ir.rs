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
    /// A compile-time constant — resolved without consulting any reader.
    Const(ConstValue),
    /// A substrate query dispatched to [`Reader::read`].
    Read(Read),
    /// A transformation over a prior slot dispatched to [`Reader::step`].
    Step {
        kind: StepKind,
        input: SlotId,
    },
    Union(SlotId, SlotId),
    Intersect(SlotId, SlotId),
    Difference(SlotId, SlotId),
}

/// Values fixed at compile time that the executor populates directly.
#[derive(Debug, Clone)]
pub(crate) enum ConstValue {
    Name { name: String, kind: NameKind },
    Ref(RefToken),
}

#[derive(Debug, Clone)]
pub(crate) enum Read {
    SearchText(String),
    ChronicleBetween { from: RefToken, to: RefToken },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StepKind {
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
        let ir = Ir::new(vec![
            Op::Const(ConstValue::Name {
                name: "observation".into(),
                kind: NameKind::Texture,
            }),
            Op::Const(ConstValue::Name {
                name: "governor.process".into(),
                kind: NameKind::Agent,
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
