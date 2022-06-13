extern crate logos;

mod compiler;
mod runner;

fn main() {
    let file = std::env::args().nth(1)
        .expect("Expected main file name arguement");
    let source = std::fs::read_to_string(&file)
        .expect(&format!("Failed to read {}", file));
    
}
