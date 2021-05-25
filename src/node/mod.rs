use crate::vertex::Vertex;
use crate::vertex_cached::VertexCached;

use std::{cell::RefCell, rc::Rc};

pub type NodeRefCell<V, D> = RefCell<Node<V, D>>;
pub type NodeRcRefCell<V, D> = Rc<NodeRefCell<V, D>>;

pub struct Node<V, D>
where
    V: Vertex,
{
    pub children: VertexCached<V>,
    pub data: D,
}

impl<V, D> Node<V, D>
where
    V: Vertex,
{
    pub fn new(key: &Rc<V>, data: D) -> Self {
        Node {
            children: VertexCached::new(&key),
            data,
        }
    }

    pub fn vertex(&self) -> &Rc<V> {
        &self.children.vertex()
    }
}
