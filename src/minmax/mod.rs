use std::cell::RefCell;
use std::rc::Rc;

use super::*;
#[derive(PartialEq, Debug)]
pub enum NodeKind {
    Minimizer,
    Maximizer,
}

#[derive(Debug)]
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

    pub fn update(&mut self, new_value: f64, edge: V::Edges) {
        if self.kind == NodeKind::Maximizer {
            if self.value < new_value {
                self.value = new_value;
                self.edge = Some(edge);
            }
        } else {
            if self.value > new_value {
                self.value = new_value;
                self.edge = Some(edge);
            }
        }
    }
}

pub struct MinMax<V>
where
    V: Vertex,
{
    pub root: NodeRcRefCell<V, NodeData<V>>,
    pub reward: Box<dyn Fn(&V) -> f64>,
    pub kind: Box<dyn Fn(&V) -> NodeKind>,
    pub depth: usize,
    pub cache: Vec<NodeRcRefCell<V, NodeData<V>>>,
}

impl<V> MinMax<V>
where
    V: Vertex,
{
    pub fn new(
        root_vertex: Rc<V>,
        reward: Box<dyn Fn(&V) -> f64>,
        kind: Box<dyn Fn(&V) -> NodeKind>,
        depth: usize,
    ) -> Self {
        let root_kind = kind(&root_vertex);
        let root_data = NodeData::new(root_kind);
        let root = Rc::new(RefCell::new(Node::new(&root_vertex, None, None, root_data)));

        let cache = Self::minmax_search(root.clone(), &reward, &kind, depth);

        MinMax {
            root,
            reward,
            kind,
            depth,
            cache,
        }
    }

    fn minmax_search(
        root: NodeRcRefCell<V, NodeData<V>>,
        reward: &Box<dyn Fn(&V) -> f64>,
        kind: &Box<dyn Fn(&V) -> NodeKind>,
        depth: usize,
    ) -> Vec<NodeRcRefCell<V, NodeData<V>>> {
        let root_kind = kind(root.borrow().vertex());
        root.borrow_mut().data = NodeData::new(root_kind);

        let mut stack = Vec::new();
        let mut cache = Vec::new();

        stack.push(root.clone());

        while let Some(node) = stack.pop() {
            let mut node_ptr = node.borrow_mut();
            match node_ptr.children.next() {
                Some((next_vertex, edge)) => {
                    stack.push(node.clone());

                    let next_depth = node_ptr.data.depth + 1;

                    if next_depth <= depth {
                        let next_kind = kind(&next_vertex);

                        let next_value = if next_vertex.is_terminal() {
                            reward(&next_vertex)
                        } else if next_kind == NodeKind::Maximizer {
                            f64::NEG_INFINITY
                        } else {
                            f64::INFINITY
                        };

                        let next_data = NodeData {
                            kind: next_kind,
                            depth: next_depth,
                            value: next_value,
                            edge: None,
                        };

                        let next_node = Rc::new(RefCell::new(Node::new(
                            &next_vertex,
                            Some(node.clone()),
                            Some(edge),
                            next_data,
                        )));

                        stack.push(next_node.clone());
                    } else {
                        node_ptr.data.update(reward(&next_vertex), edge);
                    }
                }
                None => {
                    if let (Some(p), Some(e)) = (&node_ptr.parent, &node_ptr.edge) {
                        let this_value = node_ptr.data.value;
                        p.borrow_mut().data.update(this_value, *e);
                    };

                    cache.push(node.clone());
                }
            }
        }

        return cache;
    }
}
