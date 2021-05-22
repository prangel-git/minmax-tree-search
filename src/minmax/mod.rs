use std::{cell::RefCell, rc::Rc};

use super::*;
#[derive(PartialEq)]
pub enum NodeKind {
    Minimizer,
    Maximizer,
}

pub struct NodeData {
    pub kind: NodeKind,
    pub depth: usize,
    pub value: f64,
}

impl NodeData {
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
        }
    }

    pub fn update(&mut self, new_value: f64) {
        if self.kind == NodeKind::Maximizer {
            self.value = self.value.max(new_value);
        } else {
            self.value = self.value.min(new_value)
        }
    }
}

pub struct MinMax<V>
where
    V: Vertex,
{
    pub root: NodeRcRefCell<V, NodeData>,
    pub reward: Box<dyn Fn(&V) -> f64>,
    pub kind: Box<dyn Fn(&V) -> NodeKind>,
    pub depth: usize,
    pub cache: Vec<NodeRcRefCell<V, NodeData>>,
}

impl<V> MinMax<V>
where
    V: Vertex,
{
    pub fn new(
        root: NodeRcRefCell<V, NodeData>,
        reward: Box<dyn Fn(&V) -> f64>,
        kind: Box<dyn Fn(&V) -> NodeKind>,
        depth: usize,
    ) -> Self {
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
        root: NodeRcRefCell<V, NodeData>,
        reward: &Box<dyn Fn(&V) -> f64>,
        kind: &Box<dyn Fn(&V) -> NodeKind>,
        depth: usize,
    ) -> Vec<NodeRcRefCell<V, NodeData>> {
        let root_kind = kind(root.borrow().vertex());
        root.borrow_mut().data = NodeData::new(root_kind);

        let mut stack = Vec::new();
        let mut cache = Vec::new();

        stack.push(root.clone());

        while let Some(node) = stack.pop() {
            match node.borrow_mut().children.next() {
                Some((next_vertex, _)) => {
                    stack.push(node.clone());

                    let next_depth = node.borrow().data.depth + 1;
                    if next_depth < depth {
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
                        };

                        let next_node = Rc::new(RefCell::new(Node::new(
                            &next_vertex,
                            Some(node.clone()),
                            next_data,
                        )));

                        stack.push(next_node.clone());
                        cache.push(next_node);
                    } else {
                        if let Some(p) = &node.borrow().parent {
                            p.borrow_mut().data.update(reward(&next_vertex));
                        };
                    }
                }
                None => {
                    if let Some(p) = &node.borrow().parent {
                        let this_value = node.borrow().data.value;
                        p.borrow_mut().data.update(this_value);
                    };

                    cache.push(node.clone());
                }
            }
        }

        return cache;
    }
}
