use super::CachedIterator;
use std::rc::Rc;

pub struct Node<T, Stored> {
    key: Rc<T>,
    parent: Option<Rc<Node<T, Stored>>>,
    children: CachedIterator<T>,
    depth: usize,
    value: Stored,
}

impl<T, Stored> Node<T, Stored> {
    pub fn new(
        key: Rc<T>,
        parent: Option<Rc<Node<T, Stored>>>,
        children: CachedIterator<T>,
        depth: usize,
        value: Stored,
    ) -> Self {
        Node {
            key,
            parent,
            children,
            depth,
            value,
        }
    }

    pub fn key(&self) -> &Rc<T> {
        &self.key
    }

    pub fn parent(&self) -> &Option<Rc<Node<T, Stored>>> {
        &self.parent
    }

    pub fn children(&self) -> &CachedIterator<T> {
        &self.children
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn value(&self) -> &Stored {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tree() {
        let key_1 = Rc::new(1u8);
        let key_1_children = vec![2u8, 3u8];

        let node_1 = Node::new(
            key_1,
            None,
            CachedIterator::new(Box::new(key_1_children.into_iter())),
            0,
            (),
        );

        println!("Key 1 {:}", node_1.key());
    }
}
