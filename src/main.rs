use util::{print_debug, unlikely};

mod cli;
mod compiler;
mod interpreter;
mod lexer;
mod nom;
mod optimizer;
mod parser;
mod util;

fn main() {
    let cli = cli::parse_args();
    let file = cli.file.clone();
    let input = std::fs::read_to_string(file).unwrap();

    let lexer = lexer::Lexer::new(input.clone());
    let mut parser = parser::Parser::new(lexer);
    let (ast, parse_duration) = parser.parse();

    let mut optimizer = optimizer::Optimizer::new(ast.data);
    let opt_duration = optimizer.optimize(&cli);

    let mut interpreter = interpreter::Interpreter::new(optimizer.program);

    if unlikely(cli.debug) {
        let start = std::time::Instant::now();
        interpreter.run();
        let exec_duration = start.elapsed();

        print_debug(parse_duration, opt_duration, exec_duration);
    } else {
        interpreter.run();
    }
}
