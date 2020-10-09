use super::{
    Token,
    operand_parser::operand,
    instruction_parser::AssemblerInstruction,
};

use nom::{
    IResult,
    //branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::opt,
};
use crate::assembler::label_parsers::label_declaration;

pub fn directive_declaration(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag(".")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Token::Directive {name: name.to_string()}))
}

fn directive_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, label) = opt(label_declaration)(input)?;
    let (input, name) = directive_declaration(input)?;
    let (input, operand1) = opt(operand)(input)?;
    let (input, operand2) = opt(operand)(input)?;
    let (input, operand3) = opt(operand)(input)?;
    Ok((input, AssemblerInstruction{
        opcode: None,
        directive: Some(name),
        label,
        operand1,
        operand2,
        operand3,
    }))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    directive_combined(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_directive() {
        let result = directive_combined("test: .asciiz 'Hello'");
        assert!(result.is_ok());

        let (_, directive) = result.unwrap();

        let correct_instruction = AssemblerInstruction {
            opcode: None,
            label: Some(Token::LabelDeclaration { name: "test".to_string() }),
            directive: Some(Token::Directive {name: "asciiz".to_string() }),
            operand1: Some(Token::IrString {name: "Hello".to_string() }),
            operand2: None,
            operand3: None,
        };

        assert_eq!(directive, correct_instruction);
    }
}
