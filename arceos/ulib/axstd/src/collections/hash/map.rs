use ahash::RandomState;
use axhal::misc::random;
use core::hash::Hash;
use hashbrown::hash_map as base;

pub struct HashMap<K, V> {
    map: base::HashMap<K, V, RandomState>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            map: base::HashMap::with_hasher(RandomState::with_seeds(
                random() as u64,
                random() as u64,
                random() as u64,
                random() as u64,
            )),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }
}
