pub struct InsertDelta<K, V> {
    pub key: K,
    pub value: V,
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
    pub key: K,
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

pub enum DeltaGetResult<'a, V> {
    KeyNotFound,
    NoneValue,
    Found(&'a V),
}

impl<K, V> DeltaNode<K, V>
where
    K: Ord,
{
    pub fn get(&self, key: &K) -> DeltaGetResult<'_, V> {
        match self {
            DeltaNode::Insert(insert) if insert.key == *key => DeltaGetResult::Found(&insert.value),
            DeltaNode::Update(update) if update.key == *key => DeltaGetResult::Found(&update.value),
            DeltaNode::Delete(delta) if delta.key == *key => DeltaGetResult::NoneValue,
            _ => DeltaGetResult::KeyNotFound,
        }
    }
}
