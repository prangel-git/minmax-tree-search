
pub trait ChildrenIter {
    fn children_iter(&self) -> Box<dyn Iterator<Item=Self>>;
}

#[derive(Debug)]
pub struct BinaryNode {
    pub key: u8,
}

impl ChildrenIter for BinaryNode {
    fn children_iter(&self) -> Box<dyn Iterator<Item=Self>> {
        let output = if self.key >= (1 << 7u8) {
            Vec::new()
        } else if self.key == 0 {
            let child_a = BinaryNode{key: 1};
            vec![child_a] 
        } else
        {
            let child_a = BinaryNode{key: self.key << 1};
            let child_b = BinaryNode{key: (self.key << 1) + 1};
            vec![child_a, child_b]
        };

        Box::new(output.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn depth_tree_search() {

        let mut visited = HashSet::new();

        let mut stack = Vec::new();
        let root = BinaryNode{key: 0};
        stack.push(root);

        while let Some(node) = stack.pop() {
            println!("Current node: {:08b}", &node.key);
            visited.insert(node.key);

            for child in node.children_iter() {
                stack.push(child)
            }
        }

        for k in 0..u8::MAX {
            assert_eq!(visited.contains(&k), true);
            println!("k {:08b} was visited {:} ", &k, visited.contains(&k));
        }
    }
}