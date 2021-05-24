use super::*;

use std::{cell::RefCell, rc::Rc};

pub type NodeRefCell<V, D> = RefCell<Node<V, D>>;
pub type NodeRcRefCell<V, D> = Rc<NodeRefCell<V, D>>;

pub struct Node<V, D>
where
    V: Vertex,
{
    pub parent: Option<NodeRcRefCell<V, D>>,
    pub edge: Option<V::Edges>,
    pub children: VertexCached<V>,
    pub data: D,
}

impl<V, D> Node<V, D>
where
    V: Vertex,
{
    pub fn new(
        key: &Rc<V>,
        parent: Option<Rc<RefCell<Node<V, D>>>>,
        edge: Option<V::Edges>,
        data: D,
    ) -> Self {
        Node {
            parent,
            edge,
            children: VertexCached::new(&key),
            data,
        }
    }

    pub fn vertex(&self) -> &Rc<V> {
        &self.children.vertex()
    }
}
