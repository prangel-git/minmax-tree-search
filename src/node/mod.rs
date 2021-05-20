use super::*;

use std::rc::Rc;

pub struct Node<T, Stored> 
where 
T: ChildrenIter
{
    key: Rc<T>,
    parent: Option<Rc<Node<T, Stored>>>,
    children: CachedIterator<T>,
    depth: usize,
    value: Stored,
}

impl<T, Stored> Node<T, Stored> 
where 
T: ChildrenIter
{
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
