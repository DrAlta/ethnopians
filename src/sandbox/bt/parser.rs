use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{opt, recognize},
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

use super::{ExecutionToken, Instruction};
pub fn parse_token<'a>(input: &'a str) -> IResult<&'a str, ExecutionToken, (&'a str, ErrorKind)> {
    let (tail, head) = recognize(many1(one_of("abcdefghijklmnopqrstuzwxyx_1234567890")))(input)?;
    Ok((tail, head.to_owned()))
}
/*
pub fn parse_token<'a, 'b>(
    input: &'a str,
    bt: &'b HashMap<ExecutionToken, Vec::<Instruction>>,
) -> IResult<&'a str, ExecutionToken, (&'a str, ErrorKind)>{
    let (tail, head) = recognize(many1(one_of("abcdefghijklmnopqrstuzwxyx_1234567890")))(input)?;
    if bt.contains_key(head) {
        Ok((tail, head.to_owned()))
    } else {
        Err(nom::Err::Error((head, ErrorKind::Tag)))
    }
}

pub fn gen_parse_token<'a,'b>(bt: &'a HashMap<ExecutionToken, Vec::<Instruction>>)
    -> impl FnMut(&'b str) -> IResult<&'b str, ExecutionToken, (&'b str, ErrorKind)> + 'a
{
    |input| {parse_token(input, bt)}
}
*/
pub fn parse_space<'a>(input: &'a str) -> IResult<&'a str, (), (&'a str, ErrorKind)> {
    let (tail, _) = opt(many1(char(' ')))(input)?;
    Ok((tail, ()))
}

pub fn parse_selector<'a>(
    input: &'a str,
    //    bt: &'b HashMap<ExecutionToken, Vec::<Instruction>>,
) -> IResult<&'a str, Instruction, (&'a str, ErrorKind)> {
    let (tail, (_, _, head, _, _)) = tuple((
        tag("sel{"),
        parse_space,
        separated_list1(tuple((parse_space, char(','), parse_space)), parse_token),
        parse_space,
        tag("}"),
    ))(input)?;
    Ok((
        tail,
        Instruction::Selector(head.into_iter().map(|x| x.to_owned()).collect()),
    ))
}
