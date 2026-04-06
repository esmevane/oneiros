use crate::*;

/// Placeholder — Pressure requires cross-domain computation.
pub struct PressureState;

impl PressureState {
    pub fn reduce(canon: BrainCanon, _event: &Events) -> BrainCanon {
        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
