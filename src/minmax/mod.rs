use super::*;
#[derive(PartialEq)]
pub enum NodeKind{
    Minimizer,
    Maximizer,
}

pub struct NodeData {
    pub kind: NodeKind,
    pub depth: usize,
    pub value: f64,
}

pub struct MinMax<V> where V: Vertex {
    pub root: V,
    pub depth: usize,
    pub stack: Vec<NodeRcRefCell<V, NodeData>>,
}

impl<V> MinMax<V> where V: Vertex {
    pub fn new(root: V, depth: usize) -> Self {
        MinMax {
            root,
            depth,
            stack: Vec::new(),
        }
    }
}