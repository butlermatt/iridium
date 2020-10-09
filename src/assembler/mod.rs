use crate::instructions::Opcode;
use super::assembler::{
    program_parser::*,
    assembler_errors::AssemblerError,
    instruction_parser::AssemblerInstruction,
    symbols::*,
};

pub mod opcode_parser;
pub mod register_parsers;
pub mod operand_parser;
pub mod instruction_parser;
pub mod program_parser;
pub mod directive_parser;
pub mod label_parsers;
pub mod assembler_errors;
pub mod symbols;

pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{ code: Opcode },
    Register{ reg_num: u8 },
    IntegerOperand{ value: i32 },
    LabelDeclaration{ name: String },
    LabelUsage{ name: String },
    Directive{ name: String },
    IrString{name: String},
}

#[derive(Debug,PartialEq)]
pub enum AssemblerPhase { First, Second }

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
    Data { starting_instruction: Option<u32> },
    Code { starting_instruction: Option<u32> },
    Unknown,
}

impl Default for AssemblerSection {
    fn default() -> Self {
        AssemblerSection::Unknown
    }
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(name: &str) -> Self {
        match name {
            "data" => AssemblerSection::Data { starting_instruction: None },
            "code" => AssemblerSection::Code { starting_instruction: None },
            _ => AssemblerSection::Unknown
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    /// Tracks which phase the assembler is in
    phase: AssemblerPhase,
    /// Symbol table for constants and variables
    pub symbols: SymbolTable,
    /// The read-only data section that constants are put into
    pub ro: Vec<u8>,
    /// The compiled bytecode generated from the assembly instructions
    pub bytecode: Vec<u8>,
    /// Tracks the current offset of the read-only section
    ro_offset: u32,
    /// A list of all the sections we've seen in the code.
    sections: Vec<AssemblerSection>,
    /// The current section the assembler is in
    current_section: Option<AssemblerSection>,
    /// The current instruction the assembler is converting to bytecode
    current_instruction: u32,
    /// Any errors we find along the way. At the end, we'll present them to the user
    errors: Vec<AssemblerError>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        // Runs the raw input through our `nom` parser
        match program(raw) {
            // If there were no parsing errors, we now have a Vec<AssemblyInstruction> to process.
            // `remainer` _should_ be "".
            // TODO: A check for `remainder` to make sure it is "".
            Ok((_remainder, prog)) => {
                // First get the header so we can smush it into the bytecode later.
                let mut assembled_program = self.write_pie_header();

                // Start processing the AssemblyInstruction's this is the first pass of our two pass assembler
                // We pass a read-only reference down to another function.
                self.process_first_phase(&prog);

                // If we accumulated any errors in the first pass, return them and don't try to do the second pass.
                if !self.errors.is_empty() {
                    // TODO: Can we avoid the clone here?
                    return Err(self.errors.clone());
                }

                // Make sure that we have at least one data section and one code section.
                if self.sections.len() != 2 {
                    // TODO: Detail out which ones are missing.
                    println!("Did not find exactly two sections.");
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                // Run the second pass which translates opcodes and associated operands into bytecode
                let mut body = self.process_second_phase(&prog);

                // Merge the header with the populated body vector
                assembled_program.append(&mut body);
                Ok(assembled_program)
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                Err(vec![AssemblerError::ParseError {error: e.to_string()}])
            },
        }
    }

    /// Runs the first pass of the two-pass assembling process. It looks for labels and puts them in the symbol table.
    fn process_first_phase(&mut self, p: &Program) {
        // Iterate over every instruction even though we only care able labels in this phase.
        for i in &p.instructions {
            if i.is_label() {
                if self.current_section.is_some() {
                    // If we've hit a segment header already (e.g., `.code`) then we are ok
                    self.process_label_declaration(i);
                } else {
                    // We have not hit a segment header yet, label is outside of a segment.
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound { instruction: self.current_instruction });
                }
            }

            if i.is_directive() {
                self.process_directive(i);
            }

            // This is used to keep track of which instruction we hit an error on.
            // TODO: Can this be removed/replaced?
            self.current_instruction += 1;
        }

        self.phase = AssemblerPhase::Second;
    }

    /// Runs the second pass of the assembler
    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        // Restart the counting of instructions
        self.current_instruction = 0;

        // We're going to put the bytecode meant to be executed into a separate Vec so we can do
        // some post-processing and then merge it with the header and read-only sections.
        // Optimizations, additional checks, etc.
        let mut program = vec![];

        // Same as in the first pass, but this time we care about opcodes and directives
        for i in &p.instructions {
            if i.is_opcode() {
                // Opcodes know how to properly transform themselves into 32-bits so we can just call to_bytes and append to our program
                let mut bytes = i.to_bytes(&self.symbols);
                program.append(&mut bytes);
            }
            if i.is_directive() {
                // We care about different directives than in the first pass.
                self.process_directive(i);
            }

            self.current_instruction += 1;
        }

        program
    }

    /// Handles the declaration of a label such as: hello: .asciiz 'Hello'
    fn process_label_declaration(&mut self, i: &AssemblerInstruction) {
        // Check if the label is None or String
        let name = match i.get_label_name() {
            Some(name) => name,
            None => {
                self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel { instruction: self.current_instruction });
                return;
            }
        };

        // Check if label is already in use (has an entry in the symbol table)
        // TODO: Is there a cleaner way to do this?
        if self.symbols.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }

        let symbol = Symbol::new_with_offset(name, SymbolType::Label, (self.current_instruction * 4) + 60);
        self.symbols.add_symbol(symbol);
    }

    fn process_directive(&mut self, i: &AssemblerInstruction) {
        // First check that we have a parsable name
        let directive_name = match i.get_directive_name() {
            Some(name) => name,
            None => {
                self.errors.push(AssemblerError::UnknownDirectiveFound { directive: "".to_string() });
                return;
            }
        };

        // Now check if there are any operands
        if i.has_operands() {
            // Figure out which directive it was
            match directive_name.as_ref() {
                // Null terminated string
                "asciiz" => self.handle_asciiz(i),
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound { directive: directive_name.clone() });
                    return;
                }
            }
        } else {
            // No operands so it should be a section header
            self.process_section_header(&directive_name);
        }
    }

    /// Handles a declaration of a section header, such as: .code
    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();

        if new_section == AssemblerSection::Unknown {
            self.errors.push(AssemblerError::InvalidSection {name: header_name.to_string()});
            return;
        }

        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    /// Handle a declaration of a null-terminated string: hello: .asciiz 'Hello!'
    fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
        // Constant declarations are only checked on first pass.
        if self.phase != AssemblerPhase::First { return; }

        // Operand1 will have the entire string we need to read into RO Memory
        match i.get_string_constant() {
            Some(s) => {
                match i.get_label_name() {
                    Some(name) => self.symbols.set_symbol_offset(&name, self.ro_offset),
                    None => {
                        self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel { instruction: (self.current_instruction * 4) + 60 });
                        return;
                    }
                };

                // Add string to RO memory
                for byte in s.as_bytes() {
                    self.ro.push(*byte);
                    self.ro_offset += 1;
                }

                // Add null terminator
                self.ro.push(0);
                self.ro_offset += 1;
            },
            None => println!("String constant following an .asciiz was empty")
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in PIE_HEADER_PREFIX.iter() {
            header.push(byte.clone());
        }

        while header.len() <= PIE_HEADER_LENGTH {
            header.push(0);
        }

        header
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = ".data\n.code\nload $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
        let program = asm.assemble(test_string).unwrap();
        let mut vm = VM::new();
        assert_eq!(program.len(), 93);
        vm.add_bytes(program);
        assert_eq!(vm.program.len(), 93);
    }
}