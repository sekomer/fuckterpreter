use std::time::Duration;

use crate::{
    lexer::{Lexer, Token},
    util::unlikely,
};

pub struct Parser {
    lexer: Lexer,
    loop_count: u8,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Incr(u8),
    Decr(u8),
    Next(usize),
    Prev(usize),
    Loop(Vec<ASTNode>),
    Input,
    Output,
    Set(u8),
    ScanLeft,
    ScanRight,
    Comment(char),
    NoOp,
}

#[derive(Debug, PartialEq)]
pub struct AbstractSyntaxTree {
    pub data: Vec<ASTNode>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            loop_count: 0,
        }
    }

    pub fn parse(&mut self) -> (AbstractSyntaxTree, Duration) {
        let start = std::time::Instant::now();
        let tree = self.parse_helper();
        let duration = start.elapsed();

        (tree, duration)
    }

    pub fn parse_helper(&mut self) -> AbstractSyntaxTree {
        let mut tree = Vec::new();

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Incr => {
                    let mut count = 1;
                    while let Some(Token::Incr) = self.lexer.peek_token() {
                        if count == u8::MAX {
                            break;
                        }
                        self.lexer.next_token();
                        count += 1;
                    }
                    tree.push(ASTNode::Incr(count));
                }
                Token::Decr => {
                    let mut count = 1;
                    while let Some(Token::Decr) = self.lexer.peek_token() {
                        if count == u8::MAX {
                            break;
                        }
                        self.lexer.next_token();
                        count += 1;
                    }
                    tree.push(ASTNode::Decr(count));
                }
                Token::Next => {
                    let mut count = 1;
                    while let Some(Token::Next) = self.lexer.peek_token() {
                        self.lexer.next_token();
                        count += 1;
                    }
                    tree.push(ASTNode::Next(count));
                }
                Token::Prev => {
                    let mut count = 1;
                    while let Some(Token::Prev) = self.lexer.peek_token() {
                        self.lexer.next_token();
                        count += 1;
                    }
                    tree.push(ASTNode::Prev(count));
                }
                Token::Output => tree.push(ASTNode::Output),
                Token::Input => tree.push(ASTNode::Input),
                Token::LoopStart => {
                    self.loop_count += 1;
                    tree.push(ASTNode::Loop(self.parse_helper().data));
                }
                Token::LoopEnd => {
                    if self.loop_count == 0 {
                        panic!("Unmatched loop end");
                    }

                    self.loop_count -= 1;
                    break;
                }
                Token::Other(c) => tree.push(ASTNode::Comment(c)),
            }
        }

        if unlikely(self.lexer.peek_token().is_none() && self.loop_count != 0) {
            panic!("Unmatched loop start");
        }

        AbstractSyntaxTree { data: tree }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = String::from("+-<>[],.");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let ast = parser.parse_helper();
        let mut iter = ast.data.iter();

        assert_eq!(iter.len(), 7);
        assert_eq!(iter.next(), Some(&ASTNode::Incr(1)));
        assert_eq!(iter.next(), Some(&ASTNode::Decr(1)));
        assert_eq!(iter.next(), Some(&ASTNode::Prev(1)));
        assert_eq!(iter.next(), Some(&ASTNode::Next(1)));
        assert_eq!(iter.next(), Some(&ASTNode::Loop(vec![])));
        assert_eq!(iter.next(), Some(&ASTNode::Input));
        assert_eq!(iter.next(), Some(&ASTNode::Output));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn test_loop_error_start() {
        let input = String::from("[[]");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_loop_error_end() {
        let input = String::from("[]]");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse();
    }

    #[test]
    fn test_noop() {
        let ast = AbstractSyntaxTree {
            data: vec![ASTNode::NoOp],
        };
        let mut iter = ast.data.iter();

        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(&ASTNode::NoOp));
    }
}
