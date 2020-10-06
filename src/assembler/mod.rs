use crate::instructions::Opcode;

pub mod opcode_parser;
pub mod register_parsers;
pub mod operand_parser;
pub mod instruction_parser;
pub mod program_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32}
}