use std::rc::Rc;

use crate::vertex::Vertex;
pub struct Node<V, D>
where
    V: Vertex,
{
    vertex: Rc<V>,
    visited: Vec<(Rc<V>, V::Edges)>,
    to_visit: Box<dyn Iterator<Item = V::Edges>>,
    index: usize,
    pub data: D,
}

impl<V, D> Node<V, D>
where
    V: Vertex,
{
    pub fn new(vertex: &Rc<V>, data: D) -> Self {
        Node {
            vertex: vertex.clone(),
            visited: Vec::new(),
            to_visit: vertex.edges(),
            index: 0,
            data,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn vertex(&self) -> &Rc<V> {
        &self.vertex
    }
}

impl<V, D> Iterator for Node<V, D>
where
    V: Vertex,
{
    type Item = (Rc<V>, V::Edges);

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
