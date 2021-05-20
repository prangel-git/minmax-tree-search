use std::rc::Rc;

pub struct ChildrenIterator<T> {
    visited: Vec<Rc<T>>,
    to_visit: Box<dyn Iterator<Item = T>>,
    index: usize,
}

impl<T> ChildrenIterator<T> {
    pub fn new(to_visit: Box<dyn Iterator<Item = T>>) -> Self {
        ChildrenIterator {
            visited: Vec::new(),
            to_visit,
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T> Iterator for ChildrenIterator<T> {
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
            Some(self.visited[self.index].clone())
        }
    }
}
