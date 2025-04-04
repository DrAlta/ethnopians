use nom::{
    bytes::complete::tag,
    character::streaming::multispace0,
    combinator::{map_res, recognize},
    error::ErrorKind,
    sequence::tuple,
    IResult,
};

use crate::sandbox::ai::{
    parser::{balanced_parser, forth_parser::forth_threadette_parser, space_parser},
    Thread, TaskPool,
};

mod r#if;
use r#if::If;

pub fn if_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TaskPool), (&'a str, ErrorKind)> {
    let (tail, body) = balanced_parser(
        map_res(tuple((space_parser, forth_threadette_parser)), |(_, x)| {
            Result::<(Thread, TaskPool), (&'a str, ErrorKind)>::Ok(x)
        }),
        recognize(tuple((multispace0, tag("if")))),
        recognize(tuple((multispace0, tag("then")))),
    )(input)?;

    let x = body.flatten();
    Ok((tail, x))
}

#[cfg(test)]
mod tests {
    use crate::sandbox::ai::{Instruction, StackItem};

    use super::*;

    #[test]
    fn if_parser_test() {
        let input = "if
            lit(Success)
            return
        then";
        let (tail, (body, used)) = if_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TaskPool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthIf(2),
                Instruction::ForthLit(StackItem::success()),
                Instruction::ForthReturn,
            ]
        )
    }
    #[test]
    fn nested_if_parser_test() {
        let input = "if
            lit(1) /*comment1*/
            lit(2) /*comment2*/
            if
                return
            then /*comment3*/
            lit(3)
        then";
        let (tail, (body, used)) = if_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TaskPool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthIf(5),
                Instruction::ForthLit(StackItem::Int(1)),
                Instruction::ForthLit(StackItem::Int(2)),
                Instruction::ForthIf(1),
                Instruction::ForthReturn,
                Instruction::ForthLit(StackItem::Int(3)),
            ]
        )
    }
}
