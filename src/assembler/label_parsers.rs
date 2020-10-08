use super::{
    Token,
};

use nom::{
    IResult,
    character::complete::{alphanumeric1, multispace0},
    bytes::complete::tag,
    sequence::delimited,
};

// Looks for user defined label such as `label1:`
pub fn label_declaration(input: &str) -> IResult<&str, Token> {
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = delimited(multispace0, tag(":"), multispace0)(input)?;
    Ok((input, Token::LabelDeclaration {name: name.to_string()}))
}

// Looks for a user-defined label reference, such as `@label1`
pub fn label_usage(input: &str) -> IResult<&str, Token> {
    let (input, _) = delimited(multispace0, tag("@"), multispace0)(input)?;
    let (input, name) = alphanumeric1(input)?;
    Ok((input.trim(), Token::LabelUsage {name: name.to_string()}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration("test:");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelDeclaration{name: "test".to_string()});

        let result = label_declaration("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage("@test");
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelUsage {name: "test".to_string()});

        let result = label_declaration("test");
        assert!(result.is_err());
    }
}

