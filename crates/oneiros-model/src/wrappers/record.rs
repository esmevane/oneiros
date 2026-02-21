use crate::*;

pub type Record<I, T> = Identity<I, Timestamps<T>>;

impl<I, T> Record<I, T> {
    pub fn create(inner: T) -> Self
    where
        I: From<Id>,
    {
        Self::new(I::from(Id::new()), Timestamps::now(inner))
    }

    pub fn build(
        id: I,
        inner: T,
        created_at: impl AsRef<str>,
    ) -> Result<Self, TimestampConstructionFailure> {
        Ok(Self::new(id, Timestamps::new(created_at, inner)?))
    }
}
