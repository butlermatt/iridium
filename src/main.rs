pub mod vm;
pub mod instructions;

pub mod repl;
pub mod assembler;

use clap::{
    App,
    load_yaml,
};

use std::{
    fs::File,
    path::Path,
    io::prelude::*,
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let target_file = matches.value_of("INPUT_FILE");

    if let Some(filename) = target_file {
        let program = read_file(filename);
        let mut asm = assembler::Assembler::new();
        let mut vm = vm::VM::new();
        let program = asm.assemble(&program);

        if let Ok(prog) = program {
            vm.add_bytes(prog);
            vm.run();
            std::process::exit(0);
        }
    } else {
        start_repl();
    }
}

/// Starts the REPL that will run until the user kills it.
fn start_repl() {
    let mut r = repl::REPL::new();
    r.run()
}

/// Attempts to read a file and return the contents. Exits if unable to read the file for any reason.
fn read_file(tmp: &str) -> String {
    let filename = Path::new(tmp);
    match File::open(filename) {
        Ok(mut fh) => {
            let mut contents = String::new();
            match fh.read_to_string(&mut contents) {
                Ok(_) => contents,
                Err(e) => {
                    println!("There was an error reading file: {:?}", e);
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("File not found: {:?}", e);
            std::process::exit(1);
        }
    }
}