use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use super::*;
#[derive(Clone, PartialEq, Debug)]
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
    pub root: Rc<V>,
    pub reward: Box<dyn Fn(&V) -> f64>,
    pub kind: Box<dyn Fn(&V) -> NodeKind>,
    pub depth: usize,
    pub cache: HashMap<Rc<V>, NodeRcRefCell<V, NodeData<V>>>,
}

impl<V> MinMax<V>
where
    V: Vertex + Hash + Eq,
{
    pub fn new(
        root: Rc<V>,
        reward: Box<dyn Fn(&V) -> f64>,
        kind: Box<dyn Fn(&V) -> NodeKind>,
        depth: usize,
    ) -> Self {
        let cache = HashMap::new();

        let mut output = MinMax {
            root,
            reward,
            kind,
            depth,
            cache,
        };

        output.minmax_search();
        return output;
    }

    pub fn update(&mut self, root: Rc<V>) {
        self.root = root.clone();
        self.minmax_search();
    }

    fn minmax_search(&mut self) {
        let kind = &self.kind;
        let reward = &self.reward;

        let root_data = NodeData::new(kind(&self.root));
        let root_node = if let Some(root_node_tmp) = self.cache.get(&self.root) {
            let mut root_node_ptr = root_node_tmp.borrow_mut();
            root_node_ptr.data = root_data;
            root_node_ptr.parent = None;
            root_node_ptr.edge = None;
            root_node_tmp.clone()
        } else {
            Rc::new(RefCell::new(Node::new(&self.root, None, None, root_data)))
        };

        let mut stack = Vec::new();
        let mut cache = HashMap::new();

        stack.push(root_node.clone());

        while let Some(node) = stack.pop() {
            let mut node_ptr = node.borrow_mut();
            match node_ptr.children.next() {
                Some((next_vertex, edge)) => {
                    stack.push(node.clone());

                    let next_depth = node_ptr.data.depth + 1;

                    if next_depth <= self.depth {
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

                    node_ptr.children.reset();

                    cache.insert(node_ptr.vertex().clone(), node.clone());
                }
            }
        }

        self.cache = cache;
    }
}
