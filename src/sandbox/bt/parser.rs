use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::{char, one_of}, combinator::{opt, recognize}, error::ErrorKind, multi::{many1, separated_list1}, sequence::{delimited, tuple}, IResult};

use super::{ExecutionToken, InpulseId, Instruction};
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

pub fn parse_space<'a>(input: &'a str) -> IResult<&'a str, (), (&'a str, ErrorKind)> {
    let (tail, _) = opt(many1(char(' ')))(input)?;
    Ok((tail, ()))
}

pub fn gen_parse_token<'a,'b>(bt: &'a HashMap<ExecutionToken, Vec::<Instruction>>)
    -> impl FnMut(&'b str) -> IResult<&'b str, ExecutionToken, (&'b str, ErrorKind)> + 'a
{
    |input| {parse_token(input, bt)}
}

pub fn parse_selector<'a, 'b>(
    input: &'a str, 
    bt: &'b HashMap<ExecutionToken, Vec::<Instruction>>,
) -> IResult<&'a str, Instruction, (&'a str, ErrorKind)>{
    let (tail, head) = tuple((
        tag("sel"),
        parse_space,
        separated_list1(
            tuple((
                parse_space,
                char(','),
                parse_space,
            )),
            gen_parse_token(bt)
        ), 
    ))(input)?;
    match TryInto::<Instruction>::try_into(head) {
        Ok(thing) => {
            Ok((tail, thing))
        },
        Err(_) => {
            Err(nom::Err::Error((head, ErrorKind::Tag)))
        }
    }
}