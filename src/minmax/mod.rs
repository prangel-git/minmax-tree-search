mod node_kind;
pub use node_kind::NodeKind;

mod node_data;
pub use node_data::NodeData;

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::node::Node;
use crate::vertex::Vertex;

type NodeRcRefCell<V, D> = Rc<RefCell<Node<V, D>>>;

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
        let depth = depth + 1; // Depth 0 will be reserved for data initialization

        let mut output = MinMax {
            root,
            reward,
            kind,
            depth,
            cache,
        };

        let mut cache = HashMap::new();
        output.minmax(root_clone, depth, &mut cache);
        output.cache = cache;
        return output;
    }

    pub fn update(&mut self, root: Rc<V>) {
        self.root = root.clone();
        let mut cache = HashMap::new();
        self.minmax(root, self.depth, &mut cache);
        self.cache = cache;
    }

    pub fn get_root_value_edge(&self) -> (f64, Option<V::Edges>) {
        match self.cache.get(&self.root) {
            Some(node) => (node.borrow().data.value, node.borrow().data.edge),
            None => (f64::NAN, None),
        }
    }

    fn minmax(
        &mut self,
        base: Rc<V>,
        depth: usize,
        cache: &mut HashMap<Rc<V>, NodeRcRefCell<V, NodeData<V>>>,
    ) -> f64 {
        let root = self.get_or_insert(base.clone());

        let mut root_ptr = root.borrow_mut();

        if root_ptr.data.depth >= depth {
        } else if root_ptr.vertex().is_terminal() {
            root_ptr.data.depth = usize::MAX;
            root_ptr.data.value = self.reward(&base);
        } else if depth == 1 {
            root_ptr.data.depth = 1;
            root_ptr.data.value = self.reward(&base);
        } else {
            root_ptr.reset();

            if root_ptr.data.kind == NodeKind::Maximizer {
                let mut value = f64::NEG_INFINITY;
                while let Some((child, edge)) = root_ptr.next() {
                    let child_value = self.minmax(child, depth - 1, cache);
                    if value < child_value {
                        value = child_value;
                        root_ptr.data.value = value;
                        root_ptr.data.edge = Some(edge);
                    }
                }
            } else {
                let mut value = f64::INFINITY;
                while let Some((child, edge)) = root_ptr.next() {
                    let child_value = self.minmax(child, depth - 1, cache);
                    if value > child_value {
                        value = child_value;
                        root_ptr.data.value = value;
                        root_ptr.data.edge = Some(edge);
                    }
                }
            }

            root_ptr.data.depth = depth;
        }

        cache.insert(base.clone(), root.clone());
        root_ptr.data.value
    }

    fn kind(&self, vertex: &Rc<V>) -> NodeKind {
        let kind = &self.kind;
        kind(vertex)
    }

    fn reward(&self, vertex: &Rc<V>) -> f64 {
        let reward = &self.reward;
        reward(vertex)
    }

    fn get_or_insert(&mut self, base: Rc<V>) -> NodeRcRefCell<V, NodeData<V>> {
        let node_kind = self.kind(&base);
        let output = self
            .cache
            .entry(base.clone())
            .or_insert(Rc::new(RefCell::new(Node::new(
                &base,
                NodeData::new(node_kind),
            ))));
        output.clone()
    }
}
