
pub trait ChildrenIter {
    fn children_iter(&self) -> Box<dyn Iterator<Item=Self>>;
}

pub struct BinaryNode {
    pub key: u8,
}

impl ChildrenIter for BinaryNode {
    fn children_iter(&self) -> Box<dyn Iterator<Item=Self>> {
        let output = if self.key >= (1u8 >> 8u8) {
            Vec::new()
        } else {
            let child_a = BinaryNode{key: 2 * self.key};
            let child_b = BinaryNode{key: 2 * self.key + 1};
            vec![child_a, child_b]
        };

        Box::new(output.into_iter())
    }
}