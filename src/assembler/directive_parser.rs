use super::{
    Token,
    operand_parser::operand,
    instruction_parser::AssemblerInstruction,
};

use nom::{
    IResult,
    //branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::opt,
    sequence::delimited,
};

pub fn directive_declaration(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag(".")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Token::Directive {name: name.to_string()}))
}

fn directive_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, _) = delimited(multispace0, tag("."), multispace0)(input)?;
    let (input, name) = alpha1(input)?;
    let (input, operand1) = opt(operand)(input)?;
    let (input, operand2) = opt(operand)(input)?;
    let (input, operand3) = opt(operand)(input)?;
    Ok((input, AssemblerInstruction{
        opcode: None,
        directive: Some(Token::Directive {name: name.to_string()}),
        label: None,
        operand1,
        operand2,
        operand3,
    }))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    //alt(directive_combined)(input)
    directive_combined(input)
}