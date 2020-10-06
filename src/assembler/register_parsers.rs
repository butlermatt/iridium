use super::*;

use nom::{
    IResult,
    combinator::map_res,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    sequence::delimited,
};

// named!(register <&str, Token>,
//     ws!(
//         do_parse!(
//             tag!("$") >>
//             reg_num: digit1 >>
//             (
//                 Token::Register{
//                     reg_num: reg_num.parse::<u8>().unwrap()
//                 }
//             )
//         )
//     )
// );

fn parse_register(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 10)
}

pub fn register(input: &str) -> IResult<&str, Token> {
    // Trim spaces surrounding the tag
    let (input, _) = delimited(multispace0, tag("$"), multispace0)(input)?;
    // Get 1 or more digits and pass to parsing the number
    let (input, reg_num) = map_res(digit1, parse_register)(input)?;
    Ok((input, Token::Register {reg_num}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Register {reg_num: 0});

        let result = register("0");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
    }
}