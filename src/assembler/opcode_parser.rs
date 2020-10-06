use super::*;

use nom::{
    IResult,
    bytes::complete::tag_no_case,
};

// named!(opcode_load<&str, Token>,
//     do_parse!(
//         tag!("load") >> (Token::Op{code: Opcode::LOAD})
//     )
// );

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag_no_case("load")(input)?;
    let tok = Token::Op {code: Opcode::LOAD};
    Ok((input, tok))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op {code: Opcode::LOAD});
        assert_eq!(rest, "");
        let result = opcode_load("oadl");
        assert_eq!(result.is_err(), true);
    }
}