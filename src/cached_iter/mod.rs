use std::rc::Rc;

use crate::ChildrenIter;

pub struct CachedIterator<T> 
where 
T: ChildrenIter
{
    visited: Vec<Rc<T>>,
    to_visit: Box<dyn Iterator<Item = T>>,
    index: usize,
}

impl<T> CachedIterator<T>
where 
T: ChildrenIter
{
    pub fn new(generator: &T) -> Self {
        CachedIterator {
            visited: Vec::new(),
            to_visit: generator.children_iter(),
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T> Iterator for CachedIterator<T>
where 
T: ChildrenIter
{
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.visited.len() {
            match self.to_visit.next() {
                None => None,
                Some(t) => {
                    let t_ptr = Rc::new(t);
                    self.visited.push(t_ptr.clone());
                    self.index += 1;
                    Some(t_ptr)
                }
            }
        } else {
            let index = self.index;
            self.index += 1;
            Some(self.visited[index].clone())
        }
    }
}