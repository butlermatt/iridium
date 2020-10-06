use super::{
    Token,
    opcode_parser::*,
    operand_parser::operand,
};

use nom::{
    IResult,
    combinator::opt,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match &self.opcode {
            &Token::Op { code  } => {
                results.push(code as u8);
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
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
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, ins) = instruction_combined(input)?;
    Ok((input, ins))
}

/// Handles instructions of the following form:
/// HLT
fn instruction_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, opcode) = opcode_load(input)?;
    let (input, operand1) = opt(operand)(input)?;
    let (input, operand2) = opt(operand)(input)?;
    let (input, operand3) = opt(operand)(input)?;
    Ok((input, AssemblerInstruction{
        opcode,
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
                    opcode: Token::Op {code: Opcode::LOAD},
                    operand1: Some(Token::Register {reg_num: 0}),
                    operand2: Some(Token::IntegerOperand {value: 100}),
                    operand3: None
                }
            ))
        );
    }
}