/**
 * 🦀 🦀 🦀
 * This is an alternative parser written using the nom crate.
 * Its quite a bit more concise and easier to read than the hand-written parser.
 * 🦀 🦀 🦀
*/
use nom::{
    branch,
    bytes::complete::tag,
    character::complete,
    combinator::{self, map},
    multi::{many0, many1},
    sequence, IResult,
};

use crate::parser::ASTNode;

pub fn parse(input: &str) -> IResult<&str, Vec<ASTNode>> {
    let parser_incr = map(many1(complete::char('+')), |v: Vec<char>| {
        ASTNode::Incr(v.len() as u8)
    });
    let parser_decr = map(many1(complete::char('-')), |v: Vec<char>| {
        ASTNode::Decr(v.len() as u8)
    });
    let parser_right = map(many1(complete::char('>')), |v: Vec<char>| {
        ASTNode::Next(v.len())
    });
    let parser_left = map(many1(complete::char('<')), |v: Vec<char>| {
        ASTNode::Prev(v.len())
    });
    let parser_print = map(tag("."), |_| ASTNode::Output);
    let parser_read = map(tag(","), |_| ASTNode::Input);
    let parser_loop = map(
        sequence::delimited(tag("["), parse, tag("]")),
        ASTNode::Loop,
    );
    let parser_comment = map(complete::none_of("+-<>,.[]"), ASTNode::Comment);

    many0(branch::alt((
        parser_incr,
        parser_decr,
        parser_right,
        parser_left,
        parser_print,
        parser_read,
        parser_loop,
        parser_comment,
    )))(input)
}

#[allow(dead_code)]
pub fn parse_bf(input: &str) -> Option<Vec<ASTNode>> {
    let parsed = combinator::all_consuming(parse)(input);

    match parsed {
        Ok((_, res)) => Some(res),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let program = "+-<>,.[]Comment!";
        assert_eq!(
            parse_bf(program),
            Some(vec![
                ASTNode::Incr(1),
                ASTNode::Decr(1),
                ASTNode::Prev(1),
                ASTNode::Next(1),
                ASTNode::Input,
                ASTNode::Output,
                ASTNode::Loop(vec![]),
                ASTNode::Comment('C'),
                ASTNode::Comment('o'),
                ASTNode::Comment('m'),
                ASTNode::Comment('m'),
                ASTNode::Comment('e'),
                ASTNode::Comment('n'),
                ASTNode::Comment('t'),
                ASTNode::Comment('!'),
            ])
        )
    }
}
