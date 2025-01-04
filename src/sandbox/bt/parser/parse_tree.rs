use nom::{
    branch::alt, error::ErrorKind, IResult
};


use crate::sandbox::bt::parser::{parse_action, parse_selector};

use super::{Thread, TreesUsed};


pub fn parse_tree<'a>(
    input: &'a str,
//    _prefix: &'b str
) -> IResult<
    &'a str,
    (Thread, TreesUsed),
    (&'a str, ErrorKind),
> {
//    let mut hash = HashMap::new();
    //let x = 
    alt((
        parse_selector,
        parse_action,
    ))(input)
}
