use std::rc::Rc;

pub trait Vertex<E> {
    fn edges(&self) -> Box<dyn Iterator<Item = E>>;

    fn next_vertex(&self, edge: E) -> Option<Rc<Self>>;
}