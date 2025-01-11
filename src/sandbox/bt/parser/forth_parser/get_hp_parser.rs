use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::bt::{
    parser::{ident_parser, space_parser},
    Instruction, Thread, TreePool,
};

pub fn get_hp_parser<'a>(
    input: &'a str,
) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, body, _, _)) = tuple((
        tag("get_hp"),
        space_parser,
        char('('),
        space_parser,
        ident_parser,
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        (
            vec![Instruction::ForthGetHP(body.to_owned())],
            TreePool::new(),
        ),
    ))
}
