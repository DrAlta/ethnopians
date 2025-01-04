//! Seq{
//!     a,
//!     b,
//!     c,
//! }
//! (seq{_2,_3,_4}, {_2:a, _3:b, _4:c})
//! 
use nom::{
    bytes::complete::tag, character::complete::{char, one_of}, combinator::{opt, recognize}, error::ErrorKind, multi::{many1, separated_list1}, sequence::tuple, IResult
};


use super::{ExecutionToken, Instruction, Thread, TreePool};

mod selector;
pub use selector::parse_selector;
mod parse_action;
pub use parse_action::parse_action;
mod parse_tree;
pub use parse_tree::parse_tree;

type TreesUsed = TreePool;


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

pub fn gen_parse_thread<'a,'b>(prefix: &'a str)
    -> impl FnMut(&'b str) -> IResult<&'b str, (Thread, TreesUsed), (&'b str, ErrorKind)> + 'a
{
    |input| {parse_tree(input, prefix)}
}
*/
pub fn parse_space<'a>(input: &'a str) -> IResult<&'a str, (), (&'a str, ErrorKind)> {
    let (tail, _) = opt(many1(char(' ')))(input)?;
    Ok((tail, ()))
}



pub fn parse_sequence<'a>(
    input: &'a str,
    //    bt: &'b HashMap<ExecutionToken, Vec::<Instruction>>,
) -> IResult<&'a str, Instruction, (&'a str, ErrorKind)> {
    let (tail, (_, _, head, _, _)) = tuple((
        tag("seq{"),
        parse_space,
        separated_list1(tuple((parse_space, char(','), parse_space)), parse_token),
        parse_space,
        tag("}"),
    ))(input)?;
    Ok((
        tail,
        Instruction::Sequence(head.into_iter().map(|x| x.to_owned()).collect()),
    ))
}
