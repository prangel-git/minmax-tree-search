use super::*;

use std::rc::Rc;

pub struct Node<V, E, D>
where
    V: Vertex<E>,
{
    parent: Option<Rc<Node<V, E, D>>>,
    children: VertexCached<V, E>,
    data: D,
}

impl<V, E, D> Node<V, E, D>
where
    V: Vertex<E>,
{
    pub fn new(key: &Rc<V>, parent: Option<Rc<Node<V, E, D>>>, data: D) -> Self {
        Node {
            parent,
            children: VertexCached::new(&key),
            data,
        }
    }

    pub fn vertex(&self) -> &Rc<V> {
        &self.children.vertex()
    }

    pub fn parent(&self) -> &Option<Rc<Node<V, E, D>>> {
        &self.parent
    }

    pub fn children(&self) -> &VertexCached<V, E> {
        &self.children
    }

    pub fn value(&self) -> &D {
        &self.data
    }
}
