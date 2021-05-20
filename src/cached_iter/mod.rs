use std::rc::Rc;

pub struct CachedIterator<T> {
    visited: Vec<Rc<T>>,
    to_visit: Box<dyn Iterator<Item = T>>,
    index: usize,
}

impl<T> CachedIterator<T> {
    pub fn new(to_visit: Box<dyn Iterator<Item = T>>) -> Self {
        CachedIterator {
            visited: Vec::new(),
            to_visit,
            index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T> Iterator for CachedIterator<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_iterator() {
        let integers = vec![1, 2, 3, 4];
        let integers_iter_box = Box::new(integers.clone().into_iter());

        let mut integers_cached_iter = CachedIterator::new(integers_iter_box);

        for i in 0..integers.len() {
            assert_eq!(Some(Rc::new(integers[i])), integers_cached_iter.next());
        }

        assert_eq!(None, integers_cached_iter.next());
        assert_eq!(None, integers_cached_iter.next());

        integers_cached_iter.reset();

        for i in 0..integers.len() {
            assert_eq!(Some(Rc::new(integers[i])), integers_cached_iter.next());
        }
    }
}
