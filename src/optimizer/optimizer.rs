use std::time::Duration;

use crate::cli::Cli;
use crate::parser::ASTNode;
use crate::util::likely;

pub struct Optimizer {
    pub program: Vec<ASTNode>,
}

impl Optimizer {
    pub fn new(program: Vec<ASTNode>) -> Self {
        Self { program }
    }

    pub fn optimize(&mut self, cli: &Cli) -> Duration {
        let start = std::time::Instant::now();

        if likely(!cli.f_no_optimize_scan) {
            self.scan_optimization();
        }
        if likely(!cli.f_no_optimize_clear) {
            self.clear_optimization();
        }
        if likely(!cli.f_no_optimize_loops) {
            self.remove_empty_loops();
        }

        start.elapsed()
    }

    fn remove_empty_loops_helper(nodes: &mut Vec<ASTNode>) {
        let mut i = 0;
        while i < nodes.len() {
            match &mut nodes[i] {
                ASTNode::Loop(inner_nodes) => {
                    Self::remove_empty_loops_helper(inner_nodes);

                    if inner_nodes.is_empty() {
                        nodes[i] = ASTNode::NoOp;
                    } else if inner_nodes.len() == 1 && matches!(inner_nodes[0], ASTNode::Loop(_)) {
                        nodes[i] = inner_nodes.remove(0);
                    }
                }
                _ => {}
            }

            if matches!(nodes[i], ASTNode::NoOp) {
                nodes.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn clear_optimization_helper(nodes: &mut Vec<ASTNode>) {
        for node in nodes.iter_mut() {
            match node {
                ASTNode::Loop(inner_nodes) => match inner_nodes.as_slice() {
                    [ASTNode::Decr(1)] => *node = ASTNode::Set(0),
                    _ => Self::clear_optimization_helper(inner_nodes),
                },
                _ => {}
            }
        }
    }

    fn scan_optimization_helper(nodes: &mut Vec<ASTNode>) {
        for node in nodes.iter_mut() {
            match node {
                ASTNode::Loop(inner_nodes) => {
                    if inner_nodes.len() == 1 {
                        match &inner_nodes[0] {
                            ASTNode::Next(1) => *node = ASTNode::ScanRight,
                            ASTNode::Prev(1) => *node = ASTNode::ScanLeft,
                            _ => {}
                        }
                    } else {
                        Self::scan_optimization_helper(inner_nodes);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn remove_empty_loops(&mut self) {
        Self::remove_empty_loops_helper(&mut self.program);
    }

    pub fn scan_optimization(&mut self) {
        Self::scan_optimization_helper(&mut self.program);
    }

    pub fn clear_optimization(&mut self) {
        Self::clear_optimization_helper(&mut self.program);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_scan_right_left() {
        let mut optimizer = Optimizer::new(vec![ASTNode::Loop(vec![
            ASTNode::Incr(1),
            ASTNode::Next(1),
            ASTNode::Incr(1),
            ASTNode::Next(1),
            ASTNode::Incr(1),
            ASTNode::Prev(2),
            ASTNode::Loop(vec![ASTNode::Next(1)]),
            ASTNode::Loop(vec![ASTNode::Prev(1)]),
        ])]);

        optimizer.scan_optimization();

        assert_eq!(
            optimizer.program,
            vec![ASTNode::Loop(vec![
                ASTNode::Incr(1),
                ASTNode::Next(1),
                ASTNode::Incr(1),
                ASTNode::Next(1),
                ASTNode::Incr(1),
                ASTNode::Prev(2),
                ASTNode::ScanRight,
                ASTNode::ScanLeft,
            ])]
        );
    }

    #[test]
    fn test_optimized_ast_nested() {
        let mut optimizer = Optimizer::new(vec![ASTNode::Loop(vec![
            ASTNode::Incr(100),
            ASTNode::Loop(vec![ASTNode::Next(1)]),
            ASTNode::Loop(vec![
                ASTNode::Next(1),
                ASTNode::Loop(vec![ASTNode::Next(1)]),
                ASTNode::Prev(1),
            ]),
        ])]);

        optimizer.scan_optimization();

        println!("{:?}", optimizer.program);

        assert_eq!(
            optimizer.program,
            vec![ASTNode::Loop(vec![
                ASTNode::Incr(100),
                ASTNode::ScanRight,
                ASTNode::Loop(vec![ASTNode::Next(1), ASTNode::ScanRight, ASTNode::Prev(1),]),
            ])]
        );
    }

    /**
     * * 🤟 Crazy Optimization 🤟
     */
    #[test]
    fn simplify_nested_loops() {
        let mut optimizer = Optimizer::new(vec![ASTNode::Loop(vec![
            ASTNode::Loop(vec![ASTNode::Loop(vec![])]),
            ASTNode::Loop(vec![ASTNode::Loop(vec![ASTNode::Incr(1)])]),
        ])]);

        Optimizer::remove_empty_loops_helper(&mut optimizer.program);

        assert_eq!(
            optimizer.program,
            vec![ASTNode::Loop(vec![ASTNode::Incr(1)])]
        );
    }

    #[test]
    fn remove_unused_loops() {
        let mut optimizer = Optimizer::new(vec![ASTNode::Loop(vec![ASTNode::Loop(vec![
            ASTNode::Loop(vec![ASTNode::Loop(vec![ASTNode::Loop(vec![
                ASTNode::Loop(vec![ASTNode::Loop(vec![ASTNode::Loop(vec![
                    ASTNode::Loop(vec![ASTNode::Incr(1)]),
                ])])]),
            ])])]),
        ])])]);

        Optimizer::remove_empty_loops_helper(&mut optimizer.program);

        assert_eq!(
            optimizer.program,
            vec![ASTNode::Loop(vec![ASTNode::Incr(1)]),]
        );
    }
}
