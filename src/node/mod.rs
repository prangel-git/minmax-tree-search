use super::*;

use std::rc::Rc;

pub struct Node<V, E, Stored>
where
    V: Vertex<E>,
{
    parent: Option<Rc<Node<V, E, Stored>>>,
    children: VertexCached<V, E>,
    depth: usize,
    value: Stored,
}

impl<V, E, Stored> Node<V, E, Stored>
where
    V: Vertex<E>,
{
    pub fn new(
        key: &Rc<V>,
        parent: Option<Rc<Node<V, E, Stored>>>,
        depth: usize,
        value: Stored,
    ) -> Self {
        Node {
            parent,
            children: VertexCached::new(&key),
            depth,
            value,
        }
    }

    pub fn vertex(&self) -> &Rc<V> {
        &self.children.vertex()
    }

    pub fn parent(&self) -> &Option<Rc<Node<V, E, Stored>>> {
        &self.parent
    }

    pub fn children(&self) -> &VertexCached<V, E> {
        &self.children
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn value(&self) -> &Stored {
        &self.value
    }
}
