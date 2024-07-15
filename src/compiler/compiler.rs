#![allow(dead_code)]

use crate::parser::ASTNode;

pub struct Compiler {
    nodes: Vec<ASTNode>,
}

impl Compiler {
    pub fn new(nodes: Vec<ASTNode>) -> Self {
        Compiler { nodes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let nodes = vec![ASTNode::Incr(1)];
        let compiler = Compiler::new(nodes);

        assert_eq!(compiler.nodes, vec![ASTNode::Incr(1)]);
    }
}
