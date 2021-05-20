use super::ChildrenIterator;
use std::rc::Rc;

pub struct Node<T, Stored> {
    key: Rc<T>,
    parent: Option<Rc<T>>,
    children: ChildrenIterator<T>,
    depth: usize,
    value: Stored,
}

impl<T, Stored> Node<T, Stored> {
    pub fn new(
        key: Rc<T>,
        parent: Option<Rc<T>>,
        children: ChildrenIterator<T>,
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

    pub fn parent(&self) -> &Option<Rc<T>> {
        &self.parent
    }

    pub fn children(&self) -> &ChildrenIterator<T> {
        &self.children
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn value(&self) -> &Stored {
        &self.value
    }
}
