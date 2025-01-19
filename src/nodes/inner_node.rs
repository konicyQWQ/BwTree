use crate::bw_tree::HasMinimum;

pub struct InnerNode<K>
where
    K: HasMinimum + Ord,
{
    keys: Vec<K>,
    children: Vec<usize>,
}

impl<K> InnerNode<K>
where
    K: HasMinimum + Ord,
{
    pub fn new() -> Self {
        Self {
            keys: vec![],
            children: vec![],
        }
    }

    pub fn get(&self, key: &K) -> usize {
        let idx = self.keys.partition_point(|k| key >= k);
        self.children[idx]
    }
}
