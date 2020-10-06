use super::*;

use nom::{
    IResult,
    combinator::map_res,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    sequence::delimited,
};

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

fn parse_operand(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 10)
}

pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    // Trim whitespace
    let (input, _) = delimited(multispace0, tag("#"), multispace0)(input)?;
    let (input, reg_num) = map_res(digit1, parse_operand)(input)?;
    Ok((input, Token::IntegerOperand {value: reg_num}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        // Test a valid operand
        let result = integer_operand("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::IntegerOperand {value:10});

        let result = integer_operand("10");
        assert_eq!(result.is_ok(), false);
    }
}