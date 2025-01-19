pub struct InsertDelta<K, V> {
    key: K,
    value: V,
}

impl<K, V> InsertDelta<K, V>
where
    K: Ord,
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct UpdateDelta<K, V> {
    key: K,
    value: V,
}

impl<K, V> UpdateDelta<K, V>
where
    K: Ord,
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct DeleteDelta<K> {
    key: K,
}

impl<K> DeleteDelta<K>
where
    K: Ord,
{
    pub fn new(key: K) -> Self {
        Self { key }
    }
}

pub enum DeltaNode<K, V> {
    Insert(InsertDelta<K, V>),
    Update(UpdateDelta<K, V>),
    Delete(DeleteDelta<K>),
}

impl<K, V> DeltaNode<K, V>
where
    K: Ord,
{
    /// None means not found
    /// Some(None) means the val of key is None
    /// Some(xxx) means the val of key is xxx
    pub fn get(&self, key: &K) -> Option<Option<&V>> {
        match self {
            DeltaNode::Insert(insert) => (insert.key == *key).then_some(Some(&insert.value)),
            DeltaNode::Update(update) => (update.key == *key).then_some(Some(&update.value)),
            DeltaNode::Delete(delete) => (delete.key == *key).then_some(None),
        }
    }
}
