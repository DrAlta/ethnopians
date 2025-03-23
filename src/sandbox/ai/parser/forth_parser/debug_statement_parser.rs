use nom::{
    bytes::complete::{tag, take_until},
    character::complete::char,
    error::ErrorKind,
    sequence::tuple,
    IResult,
};

use crate::sandbox::ai::{parser::space_parser, Instruction, Thread, TreePool};

pub fn debug_statement_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, _, body, _, _, _)) = tuple((
        tag("debug"),
        space_parser,
        char('('),
        space_parser,
        char('"'),
        take_until("\""),
        char('"'),
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        (vec![Instruction::Debug(body.to_owned())], TreePool::new()),
    ))
}

#[cfg(test)]
mod tests {
    use crate::sandbox::ai::Instruction;

    use super::*;
    #[test]
    fn debug_statement_parser_test() {
        let input = r#"debug("This if some %$^#^ stuff")"#;
        let (tail, body) = debug_statement_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(
            body,
            (
                vec![Instruction::Debug(r#"This if some %$^#^ stuff"#.to_owned())],
                TreePool::new()
            )
        )
    }
}
