use std::rc::Rc;

use crate::minmax::*;
use crate::Vertex;


#[derive(Debug)]
pub struct BinaryVertex {
    pub label: u8,
}

impl BinaryVertex {
    fn new(label: u8) -> Self {
        BinaryVertex { label }
    }
}

impl Vertex for BinaryVertex {
    type Edges = bool;

    fn edges(&self) -> Box<dyn Iterator<Item = Self::Edges>> {
        let edges = if self.label == 0 {
            vec![true]
        } else if self.label < (1 << 7) {
            vec![true, false]
        } else {
            vec![]
        };
        Box::new(edges.into_iter())
    }

    fn next_vertex(&self, edge: Self::Edges) -> Option<std::rc::Rc<Self>> {
        if self.label == 0 && !edge {
            None
        } else if self.label == 0 && edge {
            Some(Rc::new(Self::new(1)))
        } else if !edge {
            Some(Rc::new(Self::new(self.label << 1)))
        } else {
            Some(Rc::new(Self::new((self.label << 1) + 1)))
        }
    }

    fn is_terminal(&self) -> bool {
        self.label < (1 << 7)
    }
}

pub fn kind(vertex: &BinaryVertex) -> NodeKind {
    if vertex.label % 2 == 1 {
        NodeKind::Maximizer
    } else {
        NodeKind::Minimizer
    }
}

pub fn reward(vertex: &BinaryVertex) -> f64 {
    vertex.label as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minmax_test() {
        let root = Rc::new(BinaryVertex::new(0));
        let rew = Box::new(reward);
        let kin = Box::new(kind);
        let depth = 10usize;

        let minmax_tree = MinMax::new(root, rew, kin, depth);

        for node in minmax_tree.cache {
            println!("Vertex {:?}", node.borrow().vertex());
        }
    }
}