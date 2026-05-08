use std::collections::HashMap;

pub(crate) trait Indexable<K> {
    fn id(&self) -> K;
}

#[derive(Clone)]
pub(crate) struct EntityIndex<K, V>(HashMap<K, V>);

impl<K, V> EntityIndex<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone + Indexable<K>,
{
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    pub(crate) fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub(crate) fn set(&mut self, entity: &V) -> Option<V> {
        self.0.insert(entity.id(), entity.clone())
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &V> {
        self.0.values()
    }
}

impl<K, V> Default for EntityIndex<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}
