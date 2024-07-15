use crate::util::unlikely;

#[derive(Debug, PartialEq)]
pub enum Token {
    Incr,
    Decr,
    Next,
    Prev,
    LoopStart,
    LoopEnd,
    Input,
    Output,
    Other(char),
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn peek_token(&self) -> Option<Token> {
        if unlikely(self.position >= self.input.len()) {
            return None;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        match current_char {
            '+' => Some(Token::Incr),
            '-' => Some(Token::Decr),
            '>' => Some(Token::Next),
            '<' => Some(Token::Prev),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            ',' => Some(Token::Input),
            '.' => Some(Token::Output),
            _ => Some(Token::Other(current_char)),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if unlikely(self.position >= self.input.len()) {
            return None;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1;

        match current_char {
            '+' => Some(Token::Incr),
            '-' => Some(Token::Decr),
            '>' => Some(Token::Next),
            '<' => Some(Token::Prev),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            ',' => Some(Token::Input),
            '.' => Some(Token::Output),
            _ => Some(Token::Other(current_char)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = String::from("+-<>[],.");
        let mut lexer = Lexer::new(input.clone());
        assert_eq!(lexer.input, input);
        assert_eq!(lexer.position, 0);
        lexer.next_token();
        assert_eq!(lexer.position, 1);
    }

    #[test]
    fn test_peek_token() {
        let lexer = Lexer::new(String::from("+-<>[],."));
        assert_eq!(lexer.peek_token(), Some(Token::Incr));
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.peek_token(), Some(Token::Incr));
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.peek_token(), Some(Token::Incr));
        assert_eq!(lexer.position, 0);
    }

    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new(String::from("+-<>[],."));
        assert_eq!(lexer.next_token(), Some(Token::Incr));
        assert_eq!(lexer.next_token(), Some(Token::Decr));
        assert_eq!(lexer.next_token(), Some(Token::Prev));
        assert_eq!(lexer.next_token(), Some(Token::Next));
        assert_eq!(lexer.next_token(), Some(Token::LoopStart));
        assert_eq!(lexer.next_token(), Some(Token::LoopEnd));
        assert_eq!(lexer.next_token(), Some(Token::Input));
        assert_eq!(lexer.next_token(), Some(Token::Output));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_next_token_other() {
        let mut lexer = Lexer::new(String::from("abc123"));
        assert_eq!(lexer.next_token(), Some(Token::Other('a')));
        assert_eq!(lexer.next_token(), Some(Token::Other('b')));
        assert_eq!(lexer.next_token(), Some(Token::Other('c')));
        assert_eq!(lexer.next_token(), Some(Token::Other('1')));
        assert_eq!(lexer.next_token(), Some(Token::Other('2')));
        assert_eq!(lexer.next_token(), Some(Token::Other('3')));
        assert_eq!(lexer.next_token(), None);
    }
}
