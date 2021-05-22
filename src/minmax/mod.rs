use super::*;
#[derive(PartialEq)]
pub enum NodeKind {
    Minimizer,
    Maximizer,
}

pub struct NodeData {
    pub kind: NodeKind,
    pub depth: usize,
    pub value: f64,
}

pub struct MinMax<V>
where
    V: Vertex,
{
    pub root: V,
    pub reward: Box<dyn Fn(V) -> f64>,
    pub depth: usize,
    pub stack: Vec<NodeRcRefCell<V, NodeData>>,
}

impl<V> MinMax<V>
where
    V: Vertex,
{
    pub fn new(root: V, reward: Box<dyn Fn(V) -> f64>, depth: usize) -> Self {
        MinMax {
            root,
            reward,
            depth,
            stack: Vec::new(),
        }
    }
}
