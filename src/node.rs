struct InnerNode;

struct LeafNode;

struct DeltaNode;

enum Node {
    Leaf(LeafNode),
    Delta(DeltaNode),
    Inner(InnerNode),
}
