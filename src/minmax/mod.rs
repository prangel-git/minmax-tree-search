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

#[derive(Debug, Clone)]
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
    V: Vertex + Clone + Hash + Eq,
{
    pub fn new(
        root: Rc<V>,
        reward: Box<dyn Fn(&V) -> f64>,
        kind: Box<dyn Fn(&V) -> NodeKind>,
        depth: usize,
    ) -> Self {
        let cache = HashMap::new();
        let root_clone = root.clone();

        let mut output = MinMax {
            root,
            reward,
            kind,
            depth,
            cache,
        };

        let mut cache = HashMap::new();
        output.minmax(root_clone, depth, &mut cache);
        return output;
    }

    pub fn update(&mut self, root: Rc<V>) {
        self.root = root.clone();
        let mut cache = HashMap::new();
        self.minmax(root, self.depth, &mut cache);
        self.cache = cache;
    }

    fn minmax(
        &mut self,
        base: Rc<V>,
        depth: usize,
        cache: &mut HashMap<Rc<V>, NodeRcRefCell<V, NodeData<V>>>,
    ) -> NodeData<V> {
        let kind = &self.kind;
        let reward = &self.reward;

        let root = if let Some(node) = self.cache.get(&base).cloned() {
            let data = node.borrow().data.clone();
            if data.depth >= depth {
                cache.insert(base.clone(), node.clone());
                return data;
            }
            node.clone()
        } else {
            let node_data = NodeData::new(kind(&base));
            let node = Rc::new(RefCell::new(Node::new(&base, None, None, node_data)));
            node
        };

        let mut root_ptr = root.borrow_mut();

        if root_ptr.vertex().is_terminal() {
            root_ptr.data = NodeData {
                kind: kind(&base),
                depth: usize::MAX,
                value: reward(&base),
                edge: None,
            };
        } else if depth == 0 {
            root_ptr.data = NodeData {
                kind: kind(&base),
                depth: 0,
                value: reward(&base),
                edge: None,
            };
        } else {
            while let Some((child, edge)) = root_ptr.children.next() {
                let child_data = self.minmax(child, depth - 1, cache);
                root_ptr.data.update(child_data.value, edge);
            }
            root_ptr.children.reset();
        }

        cache.insert(base.clone(), root.clone());
        root_ptr.data.clone()
    }
}
