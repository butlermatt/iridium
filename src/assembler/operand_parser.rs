use super::*;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::delimited,
};

use crate::assembler::register_parsers::register;
use crate::assembler::label_parsers::label_usage;

// Parser for integer numbers, which we preface with a `#` in our assembly language
// eg: #100
// named!(integer_operand<&str, Token>,
//     ws!(
//         do_parse!(
//             tag!("#") >>
//             reg_num: digit >>
//             (
//                 Token::Number{value: reg_num.parse::<i32>().unwrap()}
//             )
//         )
//     )
// );

pub fn operand(input: &str) -> IResult<&str, Token> {
    alt((integer_operand, label_usage, register, irstring))(input)
}

fn parse_operand(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 10)
}

fn integer_operand(input: &str) -> IResult<&str, Token> {
    // Trim whitespace
    let (input, _) = delimited(multispace0, tag("#"), multispace0)(input)?;
    let (input, reg_num) = map_res(digit1, parse_operand)(input)?;
    Ok((input, Token::IntegerOperand {value: reg_num}))
}

fn irstring(input:&str) -> IResult<&str, Token> {
    let input = input.trim();
    let (input, content) = delimited(tag("'"), take_until("'"), tag("'"))(input)?;
    Ok((input, Token::IrString { name: content.to_string() }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        // Test a valid operand
        let result = operand("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntegerOperand {value:10});

        let result = integer_operand("10");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_string_operand() {
        let result = irstring("'This is a test'");
        assert!(result.is_ok());
        let (_, result) = result.unwrap();
        assert_eq!(result, Token::IrString {name: "This is a test".to_string() })
    }
}