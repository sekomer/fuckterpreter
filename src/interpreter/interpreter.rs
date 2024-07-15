use std::io::{Read, Write};

use crate::parser::ASTNode;

pub struct Interpreter {
    program: Vec<Instruction>,
    program_counter: usize,
    memory: Vec<u8>,
    memory_pointer: usize,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
}

pub enum Instruction {
    Incr(u8),
    Decr(u8),
    Next(usize),
    Prev(usize),
    Input,
    Output,
    BeginLoop(usize),
    EndLoop(usize),
    ScanRight,
    ScanLeft,
    Set(u8),
}

impl Interpreter {
    pub fn new(data: Vec<ASTNode>) -> Self {
        Interpreter {
            memory: vec![0; 30000],
            memory_pointer: 0,
            program: Self::compile(data),
            program_counter: 0,
            input: Box::new(std::io::stdin()),
            output: Box::new(std::io::stdout()),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_instructions(program: Vec<Instruction>) -> Self {
        Interpreter {
            memory: vec![0; 30000],
            memory_pointer: 0,
            program,
            program_counter: 0,
            input: Box::new(std::io::stdin()),
            output: Box::new(std::io::stdout()),
        }
    }

    fn compile(ast: Vec<ASTNode>) -> Vec<Instruction> {
        let mut program = Vec::new();

        for node in ast {
            match node {
                ASTNode::Incr(data) => program.push(Instruction::Incr(data)),
                ASTNode::Decr(data) => program.push(Instruction::Decr(data)),
                ASTNode::Next(data) => program.push(Instruction::Next(data)),
                ASTNode::Prev(data) => program.push(Instruction::Prev(data)),
                ASTNode::Input => program.push(Instruction::Input),
                ASTNode::Output => program.push(Instruction::Output),
                ASTNode::Loop(loop_program) => {
                    let loop_program = Self::compile(loop_program);
                    let offset = loop_program.len() + 1;
                    program.push(Instruction::BeginLoop(offset));
                    program.extend(loop_program);
                    program.push(Instruction::EndLoop(offset));
                }
                ASTNode::Set(data) => program.push(Instruction::Set(data)),
                ASTNode::ScanLeft => program.push(Instruction::ScanLeft),
                ASTNode::ScanRight => program.push(Instruction::ScanRight),
                ASTNode::NoOp => {}
                ASTNode::Comment(_) => {}
            }
        }

        program
    }

    pub fn run(&mut self) {
        while self.program_counter < self.program.len() {
            if let Some(instr) = self.program.get(self.program_counter) {
                (*self).program_counter += 1;

                match instr {
                    Instruction::Incr(data) => {
                        self.memory[self.memory_pointer] += data;
                    }
                    Instruction::Decr(data) => {
                        self.memory[self.memory_pointer] -= data;
                    }
                    Instruction::Next(data) => {
                        self.memory_pointer += *data;
                    }
                    Instruction::Prev(data) => {
                        self.memory_pointer -= *data;
                    }
                    Instruction::Input => {
                        let mut buffer = [0; 1];
                        self.input.read(&mut buffer).unwrap();
                        self.memory[self.memory_pointer] = buffer[0];
                    }
                    Instruction::Output => {
                        self.output
                            .write(&[self.memory[self.memory_pointer]])
                            .unwrap();
                    }
                    Instruction::BeginLoop(offset) => {
                        if self.memory[self.memory_pointer] == 0 {
                            self.program_counter += offset;
                        }
                    }
                    Instruction::EndLoop(offset) => {
                        if self.memory[self.memory_pointer] != 0 {
                            self.program_counter -= offset;
                        }
                    }
                    Instruction::Set(data) => {
                        self.memory[self.memory_pointer] = *data;
                    }
                    Instruction::ScanRight => {
                        while self.memory[self.memory_pointer] != 0 {
                            self.memory_pointer += 1;
                        }
                    }
                    Instruction::ScanLeft => {
                        while self.memory[self.memory_pointer] != 0 {
                            self.memory_pointer -= 1;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_new_interpreter() {
        let program = String::from("");
        let lexer = Lexer::new(program);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_helper();
        let interpreter = Interpreter::new(ast.data);

        assert_eq!(interpreter.program.len(), 0);
        assert_eq!(interpreter.memory.len(), 30000);
        assert_eq!(interpreter.memory_pointer, 0);
        assert_eq!(interpreter.program_counter, 0);
    }

    #[test]
    fn test_interpreter_parse() {
        let program = String::from("+-<>[],.");
        let lexer = Lexer::new(program);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_helper();
        let interpreter = Interpreter::new(ast.data);

        assert_eq!(interpreter.program.len(), 8);
        assert_eq!(interpreter.memory.len(), 30000);
        assert_eq!(interpreter.memory_pointer, 0);
        assert_eq!(interpreter.program_counter, 0);
    }

    #[test]
    fn test_interpreter() {
        let mut interpreter = Interpreter::new_with_instructions(vec![
            Instruction::Incr(1),
            Instruction::Output,
            Instruction::Next(2),
            Instruction::Set(42),
            Instruction::ScanLeft,
            Instruction::Set(32),
        ]);

        interpreter.run();

        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[2], 42);
        assert_eq!(interpreter.memory[1], 32);
    }
}
