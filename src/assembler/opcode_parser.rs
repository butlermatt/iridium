use super::*;

use nom::{
    IResult,
    character::complete::alpha1,
};

// named!(opcode_load<&str, Token>,
//     do_parse!(
//         tag!("load") >> (Token::Op{code: Opcode::LOAD})
//     )
// );

pub fn opcode_load(input: &str) -> IResult<&str, Token> {
    let (input, opcode) = alpha1(input)?;
    let tok = Token::Op {code: Opcode::from(opcode)};
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
        assert_eq!(result.is_ok(), true);
        let (_rest, token) = result.unwrap();
        assert_eq!(token, Token::Op {code: Opcode::IGL});
    }
}