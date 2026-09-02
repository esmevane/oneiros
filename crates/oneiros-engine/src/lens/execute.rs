use std::future::Future;
use std::pin::Pin;

use crate::*;

type MaybeSelection = Option<Result<Selection, ReaderError>>;

pub(crate) trait LensReader: Sync {
    fn read<'a>(
        &'a self,
        read: &'a LensRead,
    ) -> Pin<Box<dyn Future<Output = MaybeSelection> + Send + 'a>>;

    fn step(
        &self,
        _kind: &LensStepKind,
        _input: &Selection,
    ) -> Option<Result<Selection, ReaderError>> {
        None
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ExecuteError {
    #[error("slot {0} referenced before definition")]
    UndefinedSlot(LensSlotId),
    #[error(transparent)]
    Reader(#[from] ReaderError),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReaderError {
    #[error("reader failed: {0}")]
    Internal(String),
}

pub(crate) struct LensExecutor<'a> {
    readers: &'a [&'a dyn LensReader],
}

impl<'a> LensExecutor<'a> {
    pub(crate) fn new(readers: &'a [&'a dyn LensReader]) -> Self {
        Self { readers }
    }

    pub(crate) async fn run(
        &self,
        ir: &LensIntermediateRepresentation,
    ) -> Result<Selection, ExecuteError> {
        let mut slots: Vec<Selection> = Vec::with_capacity(ir.ops.len());

        for op in &ir.ops {
            let result = match op {
                LensOp::Const(value) => self.eval_const(value)?,
                LensOp::Read(read) => self.dispatch_read(read).await?,
                LensOp::Step { kind, input } => {
                    let input_selection = self.resolve(&slots, *input)?.clone();
                    self.dispatch_step(kind, &input_selection)?
                }
                LensOp::Union(left, right) => {
                    let left = self.resolve(&slots, *left)?;
                    let right = self.resolve(&slots, *right)?;
                    left.union(right)
                }
                LensOp::Intersect(left, right) => {
                    let left = self.resolve(&slots, *left)?;
                    let right = self.resolve(&slots, *right)?;
                    left.intersect(right)
                }
                LensOp::Difference(left, right) => {
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

    fn eval_const(&self, value: &LensConstValue) -> Result<Selection, ExecuteError> {
        let mut selection = Selection::new();
        match value {
            LensConstValue::Name { name, kind } => {
                selection.insert(Hit::Name(NameHit {
                    name: name.clone(),
                    kind: *kind,
                    timestamp: Timestamp::now(),
                    relevance: Relevance::Unknown,
                }));
            }
            LensConstValue::Ref(reference) => {
                selection.insert(Hit::Entity(EntityHit {
                    entity_ref: reference.inner().clone(),
                    timestamp: Timestamp::now(),
                    relevance: Relevance::Unknown,
                }));
            }
        }
        Ok(selection)
    }

    async fn dispatch_read(&self, read: &LensRead) -> Result<Selection, ExecuteError> {
        for reader in self.readers {
            if let Some(result) = reader.read(read).await {
                return Ok(result?);
            }
        }
        Ok(Selection::new())
    }

    fn dispatch_step(
        &self,
        kind: &LensStepKind,
        input: &Selection,
    ) -> Result<Selection, ExecuteError> {
        for reader in self.readers {
            if let Some(result) = reader.step(kind, input) {
                return Ok(result?);
            }
        }
        Ok(Selection::new())
    }

    fn resolve<'b>(
        &self,
        slots: &'b [Selection],
        slot: LensSlotId,
    ) -> Result<&'b Selection, ExecuteError> {
        slots.get(slot.0).ok_or(ExecuteError::UndefinedSlot(slot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EmptyReader;
    impl LensReader for EmptyReader {
        fn read<'a>(
            &'a self,
            _read: &'a LensRead,
        ) -> Pin<Box<dyn Future<Output = Option<Result<Selection, ReaderError>>> + Send + 'a>>
        {
            Box::pin(async { Some(Ok(Selection::new())) })
        }
    }

    #[tokio::test]
    async fn executor_returns_empty_for_no_readers() {
        let readers: Vec<&dyn LensReader> = vec![];
        let executor = LensExecutor::new(&readers);
        let ir = LensIntermediateRepresentation::new(vec![LensOp::Read(LensRead::SearchText(
            "hello".into(),
        ))]);
        let result = executor.run(&ir).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn executor_dispatches_to_first_claiming_reader() {
        let empty = EmptyReader;
        let readers: Vec<&dyn LensReader> = vec![&empty];
        let executor = LensExecutor::new(&readers);
        let ir = LensIntermediateRepresentation::new(vec![LensOp::Read(LensRead::SearchText(
            "hello".into(),
        ))]);
        let result = executor.run(&ir).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn executor_runs_set_operators() {
        let empty = EmptyReader;
        let readers: Vec<&dyn LensReader> = vec![&empty];
        let executor = LensExecutor::new(&readers);
        let ir = LensIntermediateRepresentation::new(vec![
            LensOp::Read(LensRead::SearchText("a".into())),
            LensOp::Read(LensRead::SearchText("b".into())),
            LensOp::Union(LensSlotId(0), LensSlotId(1)),
        ]);
        let result = executor.run(&ir).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn executor_rejects_undefined_slot() {
        let readers: Vec<&dyn LensReader> = vec![];
        let executor = LensExecutor::new(&readers);
        let ir =
            LensIntermediateRepresentation::new(vec![LensOp::Union(LensSlotId(0), LensSlotId(1))]);
        let err = executor.run(&ir).await.unwrap_err();
        assert!(matches!(err, ExecuteError::UndefinedSlot(LensSlotId(0))));
    }
}
