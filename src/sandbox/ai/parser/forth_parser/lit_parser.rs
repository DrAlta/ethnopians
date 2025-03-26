use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, one_of},
    combinator::{map_res, opt, recognize},
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{terminated, tuple},
    IResult,
};

use crate::sandbox::ai::{
    parser::{ident_parser, space_parser},
    Instruction, StackItem, Thread, TreePool,
};

pub fn lit_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, body, _, _)) = tuple((
        tag("lit"),
        space_parser,
        char('('),
        space_parser,
        alt((
            coord_parser,
            map_res(
                recognize(tuple((
                    opt(char('-')),
                    many1(terminated(one_of("0123456789"), many0(char('_')))),
                ))),
                |out| {
                    let Ok(int) = i32::from_str_radix(&str::replace(out, "_", ""), 10) else {
                        return Err(());
                    };
                    Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::Int(int)))
                },
            ),
            map_res(tag_no_case("Success"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::success()))
            }),
            map_res(tag_no_case("failure"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::failure()))
            }),
            map_res(tag_no_case("init"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::init()))
            }),
            map_res(tag_no_case("true"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::True))
            }),
            map_res(tag_no_case("false"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::False))
            }),
            map_res(tuple((char('"'), ident_parser, char('"'))), |(_, x, _)| {
                Ok::<Instruction, ()>(Instruction::ForthLit(x.into()))
            }),
        )),
        space_parser,
        char(')'),
    ))(input)?;
    Ok((tail, (vec![body], TreePool::new())))
}

pub fn coord_parser<'a>(input: &'a str) -> IResult<&'a str, Instruction, (&'a str, ErrorKind)> {
    //let (tail, (_, _, _, _, body, _, _)) =
    map_res(
        tuple((
            tuple((char('x'), space_parser, char(':'))),
            space_parser,
            map_res(
                recognize(tuple((
                    opt(char('-')),
                    many1(terminated(one_of("0123456789"), many0(char('_')))),
                ))),
                |out| {
                    let Ok(int) = i32::from_str_radix(&str::replace(out, "_", ""), 10) else {
                        return Err(());
                    };
                    Ok::<i32, ()>(int)
                },
            ),
            space_parser,
            char(','),
            space_parser,
            tuple((char('y'), space_parser, char(':'))),
            space_parser,
            map_res(
                recognize(tuple((
                    opt(char('-')),
                    many1(terminated(one_of("0123456789"), many0(char('_')))),
                ))),
                |out| {
                    let Ok(int) = i32::from_str_radix(&str::replace(out, "_", ""), 10) else {
                        return Err(());
                    };
                    Ok::<i32, ()>(int)
                },
            ),
        )),
        |(_, _, x, _, _, _, _, _, y)| {
            Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::Coord { x, y }))
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::sandbox::ai::Instruction;

    use super::*;
    #[test]
    fn coord_test() {
        let input = "x:  6,y : 9";
        let (tail, body) = coord_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(body, Instruction::ForthLit(StackItem::Coord { x: 6, y: 9 }))
    }

    #[test]
    fn coord_lit_test() {
        let input = "lit(x:  -10, y: 5)";
        let (tail, (body, used)) = lit_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![Instruction::ForthLit(StackItem::Coord { x: -10, y: 5 })]
        )
    }
    #[test]
    fn success_test() {
        let input = "lit(Success)";
        let (tail, (body, used)) = lit_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(body, vec![Instruction::ForthLit(StackItem::success())])
    }
    #[test]
    fn int_test() {
        let input = "lit(1)";
        let (tail, (body, used)) = lit_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(body, vec![Instruction::ForthLit(StackItem::Int(1))])
    }
    #[test]
    fn string_test() {
        let input = "lit(\"one\")";
        let (tail, (body, used)) = lit_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(body, vec![Instruction::ForthLit("one".into())])
    }
}
