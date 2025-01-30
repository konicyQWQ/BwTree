use crate::bw_tree::HasMinimum;
use crate::nodes::delta_node::DeltaNode;
use crate::nodes::inner_node::InnerNode;
use crate::nodes::leaf_node::LeafNode;

pub mod delta_node;
pub mod inner_node;
pub mod leaf_node;

pub enum Node<K, V>
where
    K: HasMinimum + Ord + Clone,
{
    Leaf(LeafNode<K, V>),
    Delta(DeltaNode<K, V>),
    Inner(InnerNode<K>),
}
