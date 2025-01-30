use crate::nodes::delta_node::InsertDelta;

#[derive(Clone)]
pub struct LeafNode<K, V>
where
    K: Ord + Clone,
{
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> LeafNode<K, V>
where
    K: Ord + Clone,
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

pub struct LeafNodeBuilder<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> LeafNodeBuilder<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    pub fn add_delta(&mut self, delta: &InsertDelta<K, V>) {
        self.keys.push(delta.key.clone());
        self.values.push(delta.value.clone());
    }

    pub fn add_node(&mut self, node: &LeafNode<K, V>) {
        self.keys.append(&mut node.keys.clone());
        self.values.append(&mut node.values.clone());
    }

    pub fn build(self) -> LeafNode<K, V> {
        let mut indices: Vec<usize> = (0..self.keys.len()).collect();

        indices.sort_by(|&i, &j| self.keys[i].cmp(&self.keys[j]));

        let sorted_keys: Vec<K> = indices.iter().map(|&i| self.keys[i].clone()).collect();
        let sorted_values: Vec<V> = indices.iter().map(|&i| self.values[i].clone()).collect();

        LeafNode {
            keys: sorted_keys,
            values: sorted_values,
        }
    }
}
