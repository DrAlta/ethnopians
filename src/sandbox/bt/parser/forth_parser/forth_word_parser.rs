use nom::{branch::alt, combinator::map_res, error::ErrorKind, IResult};

use crate::sandbox::bt::{parser::token_parser, Instruction, Thread, TreePool};


pub fn forth_threadette_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, head) = alt((
        map_res(
            token_parser,
            |x| {
                Ok::<(Thread, TreePool), ()>(
                match x {
                    crate::sandbox::bt::parser::Thingie::Token(tokan) => (
                        vec![Instruction::ForthCall(token, 0)], 
                        TreePool::new()
                    ),
                    crate::sandbox::bt::parser::Thingie::Tree(vec, hash_map) => (vec, hash_map),
                })
            }
        ),
        |x| {Ok((x, (Thread::new(), TreePool::new())))}
    ))(input)?;
    todo!()

}