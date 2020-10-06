pub mod vm;
pub mod instructions;

pub mod repl;
pub mod assembler;

extern crate nom;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
