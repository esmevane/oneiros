use crate::*;

pub(crate) struct Executor<'a> {
    readers: &'a [&'a dyn Reader],
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ExecuteError {
    #[error("slot {0} referenced before definition")]
    UndefinedSlot(SlotId),
    #[error(transparent)]
    Reader(#[from] ReaderError),
}

impl<'a> Executor<'a> {
    pub(crate) fn new(readers: &'a [&'a dyn Reader]) -> Self {
        Self { readers }
    }

    pub(crate) fn run(&self, ir: &Ir) -> Result<Selection, ExecuteError> {
        let mut slots: Vec<Selection> = Vec::with_capacity(ir.ops.len());

        for op in &ir.ops {
            let result = match op {
                Op::Read(read) => self.dispatch(read)?,
                Op::Union(left, right) => {
                    let left = self.resolve(&slots, *left)?;
                    let right = self.resolve(&slots, *right)?;
                    left.union(right)
                }
                Op::Intersect(left, right) => {
                    let left = self.resolve(&slots, *left)?;
                    let right = self.resolve(&slots, *right)?;
                    left.intersect(right)
                }
                Op::Difference(left, right) => {
                    let left = self.resolve(&slots, *left)?;
                    let right = self.resolve(&slots, *right)?;
                    left.difference(right)
                }
            };
            slots.push(result);
        }

        let result_slot = ir.result_slot();
        self.resolve(&slots, result_slot).cloned()
    }

    fn dispatch(&self, read: &Read) -> Result<Selection, ExecuteError> {
        for reader in self.readers {
            if let Some(result) = reader.read(read) {
                return Ok(result?);
            }
        }
        Ok(Selection::new())
    }

    fn resolve<'b>(
        &self,
        slots: &'b [Selection],
        slot: SlotId,
    ) -> Result<&'b Selection, ExecuteError> {
        slots.get(slot.0).ok_or(ExecuteError::UndefinedSlot(slot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EmptyReader;
    impl Reader for EmptyReader {
        fn read(&self, _read: &Read) -> Option<Result<Selection, ReaderError>> {
            Some(Ok(Selection::new()))
        }
    }

    #[test]
    fn executor_returns_empty_for_no_readers() {
        let readers: Vec<&dyn Reader> = vec![];
        let executor = Executor::new(&readers);
        let ir = Ir::new(vec![Op::Read(Read::SearchText("hello".into()))]);
        let result = executor.run(&ir).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn executor_dispatches_to_first_claiming_reader() {
        let empty = EmptyReader;
        let readers: Vec<&dyn Reader> = vec![&empty];
        let executor = Executor::new(&readers);
        let ir = Ir::new(vec![Op::Read(Read::SearchText("hello".into()))]);
        let result = executor.run(&ir).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn executor_runs_set_operators() {
        let empty = EmptyReader;
        let readers: Vec<&dyn Reader> = vec![&empty];
        let executor = Executor::new(&readers);
        let ir = Ir::new(vec![
            Op::Read(Read::SearchText("a".into())),
            Op::Read(Read::SearchText("b".into())),
            Op::Union(SlotId(0), SlotId(1)),
        ]);
        let result = executor.run(&ir).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn executor_rejects_undefined_slot() {
        let readers: Vec<&dyn Reader> = vec![];
        let executor = Executor::new(&readers);
        let ir = Ir::new(vec![Op::Union(SlotId(0), SlotId(1))]);
        let err = executor.run(&ir).unwrap_err();
        assert!(matches!(err, ExecuteError::UndefinedSlot(SlotId(0))));
    }
}
