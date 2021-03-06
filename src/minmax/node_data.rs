use super::NodeKind;
use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct NodeData<V>
where
    V: Vertex,
{
    pub kind: NodeKind,
    pub depth: usize,
    pub value: f64,
    pub edge: Option<V::Edges>,
}

impl<V> NodeData<V>
where
    V: Vertex,
{
    pub fn new(kind: NodeKind) -> Self {
        let value = if kind == NodeKind::Maximizer {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
        NodeData {
            kind,
            depth: 0,
            value,
            edge: None,
        }
    }
}
