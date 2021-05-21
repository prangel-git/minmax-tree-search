use std::rc::Rc;

use super::*;

pub struct VertexCached<V, E>
where
    V: Vertex<E>,
{
    vertex: Rc<V>,
    visited: Vec<(Rc<V>, E)>,
    to_visit: Box<dyn Iterator<Item = E>>,
    index: usize,
}

impl<V, E> VertexCached<V, E>
where
    V: Vertex<E>,
{
    pub fn new(vertex: &Rc<V>) -> Self {
        VertexCached {
            vertex: vertex.clone(),
            visited: Vec::new(),
            to_visit: vertex.edges(),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn vertex(&self) -> &Rc<V> {
        &self.vertex
    }
}

impl<V, E> Iterator for VertexCached<V, E>
where
    E: Copy,
    V: Vertex<E>,
{
    type Item = (Rc<V>, E);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.visited.len() {
            match self.to_visit.next() {
                None => None,
                Some(e) => match self.vertex.next_vertex(e) {
                    Some(v) => {
                        let output = (v, e);
                        self.visited.push(output.clone());
                        self.index += 1;
                        Some(output)
                    }
                    None => None,
                },
            }
        } else {
            let index = self.index;
            self.index += 1;
            Some(self.visited[index].clone())
        }
    }
}
