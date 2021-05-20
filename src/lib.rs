mod cached_iter;
pub use cached_iter::CachedIterator;

mod node;
pub use node::Node;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
