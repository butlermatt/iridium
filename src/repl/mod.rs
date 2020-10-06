use std::{self, io::{self, Write}};

use crate::vm::VM;
use crate::assembler::program_parser::*;

use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> Self {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how to create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command.
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin.read_line(&mut buffer).expect("unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".program" => {
                    println!("Listing instructions currently in the VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing");
                },
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue
                        }
                    };

                    self.vm.program.append(&mut program.to_bytes());
                    self.vm.run_once();
                }
            }
        }
    }

    #[allow(dead_code)]
    /// Accepts a hexadecimal string without the prefix `0x` and returns a Vec of u8
    /// Example for a LOAD command: 01 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }
}