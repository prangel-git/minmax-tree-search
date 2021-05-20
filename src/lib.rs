mod children_iterator;
pub use children_iterator::ChildrenIterator;

mod node;
pub use node::Node;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
