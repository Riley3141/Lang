extern crate logos;

mod syntax;
mod parser;
mod compiler;
mod runner;

fn main() {
    let file = std::env::args().nth(1)
        .expect("Expected main file name arguement");
    let source = std::fs::read_to_string(&file)
        .expect(&format!("Failed to read {}", file));
    let mut parser = parser::Parser::new();
    parser.parse(&source);
    let program = compiler::compile(parser.parsed);
    runner::run(program);
}
