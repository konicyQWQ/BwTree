use crate::lockfree_list::LockFreeList;
use crate::mapping_table::MappingTable;
use crate::nodes::delta_node::{DeltaNode, InsertDelta};
use crate::nodes::leaf_node::{LeafNode, LeafNodeBuilder};
use crate::nodes::Node;
use crossbeam::epoch;
use crossbeam::epoch::Guard;

pub trait HasMinimum {
    const MINIMUM: Self;
}

pub struct BwTree<K, V>
where
    K: HasMinimum + Ord + Clone,
{
    mapping_table: MappingTable<K, V>,
    root_id: usize,
}

impl<K, V> BwTree<K, V>
where
    K: HasMinimum + Ord + Clone,
    V: Clone,
{
    pub fn new() -> Result<Self, anyhow::Error> {
        let mapping_table = MappingTable::new();
        let root_id = mapping_table.new_page()?;
        let root = Node::Leaf(LeafNode::new());

        mapping_table.get(root_id).push_front(root);

        Ok(Self {
            root_id,
            mapping_table,
        })
    }

    pub fn insert(&self, key: K, value: V) -> Result<(), anyhow::Error> {
        let node = Node::Delta(DeltaNode::Insert(InsertDelta::new(key, value)));

        // TODO: check whether node is inner node
        let root_list = self.mapping_table.get(self.root_id);
        root_list.push_front(node);
        Ok(())
    }

    pub fn get<'a>(&'a self, key: &K, guard: &'a Guard) -> Option<&V> {
        let mut node_list = self.mapping_table.get(self.root_id);
        loop {
            match node_list.get(key, &guard) {
                TreeSearch::NextNode(id) => {
                    node_list = self.mapping_table.get(id);
                }
                TreeSearch::Val(val) => {
                    return val;
                }
            }
        }
    }

    fn consolidation_impl(&self, list: &LockFreeList<Node<K, V>>, guard: &Guard) -> Node<K, V> {
        let mut builder = LeafNodeBuilder::new();

        for node in list.iter_with_guard(guard) {
            match node {
                Node::Leaf(leaf) => {
                    builder.add_node(leaf);
                }
                Node::Inner(_) => todo!(),
                Node::Delta(delta) => match delta {
                    DeltaNode::Insert(delta) => {
                        builder.add_delta(delta);
                    }
                    DeltaNode::Update(_) => todo!(),
                    DeltaNode::Delete(_) => todo!(),
                },
            }
        }

        Node::Leaf(builder.build())
    }

    pub fn consolidation(&self, page_id: usize) {
        let guard = epoch::pin();
        self.mapping_table
            .get(page_id)
            .replace(|current| self.consolidation_impl(current, &guard), &guard);
    }
}

pub enum TreeSearch<'a, V> {
    NextNode(usize),
    Val(Option<&'a V>),
}

impl<K, V> LockFreeList<Node<K, V>>
where
    K: Ord + HasMinimum + Clone,
{
    pub fn get<'a>(&'a self, key: &K, guard: &'a Guard) -> TreeSearch<'_, V> {
        for node in self.iter_with_guard(guard) {
            match node {
                Node::Delta(delta_node) => {
                    if let Some(v) = delta_node.get(key) {
                        return TreeSearch::Val(v);
                    }
                }
                Node::Leaf(leaf_node) => return TreeSearch::Val(leaf_node.get(key)),
                Node::Inner(inner_node) => return TreeSearch::NextNode(inner_node.get(key)),
            }
        }

        unreachable!()
    }
}

mod tests {
    use super::*;
    use crossbeam::epoch;

    impl HasMinimum for i32 {
        const MINIMUM: i32 = i32::MIN;
    }

    #[test]
    fn test_insert_get() -> Result<(), anyhow::Error> {
        let bw_tree = BwTree::new()?;

        let guard = epoch::pin();
        bw_tree.insert(1, 2)?;
        bw_tree.insert(2, 4)?;

        assert_eq!(bw_tree.get(&1, &guard), Some(&2));
        assert_eq!(bw_tree.get(&2, &guard), Some(&4));
        assert_eq!(bw_tree.get(&3, &guard), None);

        bw_tree.insert(3, 6)?;
        assert_eq!(bw_tree.get(&1, &guard), Some(&2));
        assert_eq!(bw_tree.get(&2, &guard), Some(&4));
        assert_eq!(bw_tree.get(&3, &guard), Some(&6));

        Ok(())
    }

    #[test]
    fn test_insert_consolidation() -> Result<(), anyhow::Error> {
        let bw_tree = BwTree::new()?;

        let guard = epoch::pin();
        bw_tree.insert(1, 2)?;
        bw_tree.insert(2, 4)?;

        assert_eq!(bw_tree.get(&1, &guard), Some(&2));
        assert_eq!(bw_tree.get(&2, &guard), Some(&4));
        assert_eq!(bw_tree.get(&3, &guard), None);

        bw_tree.insert(3, 6)?;
        assert_eq!(bw_tree.get(&1, &guard), Some(&2));
        assert_eq!(bw_tree.get(&2, &guard), Some(&4));
        assert_eq!(bw_tree.get(&3, &guard), Some(&6));

        bw_tree.consolidation(0);
        assert_eq!(bw_tree.get(&1, &guard), Some(&2));
        assert_eq!(bw_tree.get(&2, &guard), Some(&4));
        assert_eq!(bw_tree.get(&3, &guard), Some(&6));
        assert_eq!(bw_tree.get(&4, &guard), None);

        Ok(())
    }
}
