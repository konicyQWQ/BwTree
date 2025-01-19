pub struct LeafNode<K, V>
where
    K: Ord,
{
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> LeafNode<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.keys.binary_search(key).ok().map(|i| &self.values[i])
    }
}
