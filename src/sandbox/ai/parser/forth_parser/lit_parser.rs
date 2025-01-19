use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    error::ErrorKind,
    multi::{many0, many1},
    sequence::{terminated, tuple},
    IResult,
};

use crate::sandbox::ai::{parser::{ident_parser, space_parser}, Instruction, StackItem, Thread, TreePool};

pub fn lit_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, body, _, _)) = tuple((
        tag("lit"),
        space_parser,
        char('('),
        space_parser,
        alt((
            map_res(
                recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
                |out| {
                    let Ok(int) = i32::from_str_radix(&str::replace(out, "_", ""), 10) else {
                        return Err(());
                    };
                    Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::Int(int)))
                },
            ),
            map_res(tag_no_case("Success"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::Success))
            }),
            map_res(tag_no_case("failure"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::Failure))
            }),
            map_res(tag_no_case("init"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::Init))
            }),
            map_res(tag_no_case("true"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::True))
            }),
            map_res(tag_no_case("false"), |_| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::False))
            }),
            map_res(tuple((
                char('"'),
                ident_parser,
                char('"'),
            )), |(_,x,_)| {
                Ok::<Instruction, ()>(Instruction::ForthLit(StackItem::String(x.to_owned())))
            }),
        )),
        space_parser,
        char(')'),
    ))(input)?;
    Ok((tail, (vec![body], TreePool::new())))
}

#[cfg(test)]
mod tests {
    use crate::sandbox::ai::Instruction;

    use super::*;

    #[test]
    fn success_test() {
        let input = "lit(Success)";
        let (tail, (body, used)) = lit_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(body, vec![Instruction::ForthLit(StackItem::Success)])
    }
}
