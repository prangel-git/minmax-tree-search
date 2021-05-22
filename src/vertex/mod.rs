use std::rc::Rc;

pub trait Vertex {
    type Edges: Copy;

    fn edges(&self) -> Box<dyn Iterator<Item = Self::Edges>>;

    fn next_vertex(&self, edge: Self::Edges) -> Option<Rc<Self>>;

    fn is_terminal(&self) -> bool;
}
