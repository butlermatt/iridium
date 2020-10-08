use super::{
    Token,
    opcode_parser::*,
    operand_parser::operand,
    directive_parser::directive,
    label_parsers::label_declaration,
};

use nom::{
    IResult,
    branch::alt,
    combinator::opt,
};
use crate::assembler::SymbolTable;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results: Vec<u8> = vec![];
        match &self.opcode {
            Some(Token::Op { code  }) => {
                results.push(*code as u8);
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols);
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        return results;
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn label_name(&self) -> Option<String> {
        if let Some(l) = &self.label {
            match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None
            }
        } else {
            None
        }
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num)
            },
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let b1 = converted;
                let b2 = converted >> 8;
                results.push(b2 as u8);
                results.push(b1 as u8);
            },
            Token::LabelUsage { name } => {
                if let Some(value) = symbols.symbol_value(name) {
                    let converted = value as u16;
                    let b1 = converted;
                    let b2 = converted >> 8;
                    results.push(b2 as u8);
                    results.push(b1 as u8);
                } else {
                    println!("No value found for {:?}", name);
                }
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    alt((instruction_combined, directive))(input)
}

/// Handles instructions of the following form:
/// HLT
fn instruction_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, label) = opt(label_declaration)(input)?;
    let (input, opcode) = opcode_load(input)?;
    let (input, operand1) = opt(operand)(input)?;
    let (input, operand2) = opt(operand)(input)?;
    let (input, operand3) = opt(operand)(input)?;
    Ok((input, AssemblerInstruction{
        opcode: Some(opcode),
        directive: None,
        label,
        operand1,
        operand2,
        operand3,
    }))
}
//
// fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
//     let input = input.trim();
//     let (input, opcode) = opcode_load(input)?;
//     let (input, operand1) = operand(input)?;
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::Opcode;

    #[test]
    fn test_parse_instruction_from_one() {
        let result = instruction("load $0 #100\n");
        assert_eq!(
            result, Ok((
                "",
                AssemblerInstruction{
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op {code: Opcode::LOAD}),
                    operand1: Some(Token::Register {reg_num: 0}),
                    operand2: Some(Token::IntegerOperand {value: 100}),
                    operand3: None
                }
            ))
        );
    }
}