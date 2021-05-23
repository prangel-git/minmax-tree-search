mod vertex;
pub use vertex::Vertex;

mod vertex_cached;
pub use vertex_cached::VertexCached;

mod node;
pub use node::*;

pub mod minmax;

pub mod binary_tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
