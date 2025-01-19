use crate::bw_tree::HasMinimum;
use crate::lockfree_list::LockFreeList;
use crate::nodes::Node;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

const MAPPING_TABLE_SIZE: usize = 1 << 20;

pub struct MappingTable<K, V>
where
    K: Ord + HasMinimum,
{
    map: Vec<LockFreeList<Node<K, V>>>,
    next_id: AtomicUsize,
}

impl<K, V> MappingTable<K, V>
where
    K: Ord + HasMinimum,
{
    pub fn new() -> Self {
        Self {
            map: (0..MAPPING_TABLE_SIZE).map(|_| LockFreeList::new()).collect(),
            next_id: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, id: usize) -> &LockFreeList<Node<K, V>> {
        debug_assert!(id < MAPPING_TABLE_SIZE);
        &self.map[id]
    }

    pub fn new_page(&self) -> usize {
        self.next_id.fetch_add(1, Relaxed)
    }
}
