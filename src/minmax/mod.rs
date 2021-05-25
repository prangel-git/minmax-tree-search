mod node_kind;
pub use node_kind::NodeKind;

mod node_data;
pub use node_data::NodeData;

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::node::Node;
use crate::node::NodeRcRefCell;
use crate::vertex::Vertex;

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
