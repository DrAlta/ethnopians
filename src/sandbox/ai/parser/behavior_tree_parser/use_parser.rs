use nom::{
    bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, IResult,
};

use crate::sandbox::ai::{
    parser::{behavior_tree_parser::Thingie, ident_parser, space_parser},
    Instruction, TaskPool,
};

pub fn use_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, (_, _, _, _, item_a, _, item_b, _, _)) = tuple((
        tag("use"),
        space_parser,
        char('('),
        space_parser,
        ident_parser,
        tuple((space_parser, char(','), space_parser)),
        ident_parser,
        space_parser,
        char(')'),
    ))(input)?;
    Ok((
        tail,
        Thingie::Tree(
            vec![Instruction::Use(item_a.to_owned(), item_b.to_owned())],
            TaskPool::new(),
        ),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_parser_test() {
        let (_, Thingie::Tree(i, _db)) = use_parser("use ( stone , stick)").unwrap() else {
            panic!()
        };
        assert_eq!(
            i,
            vec![Instruction::Use("stone".to_owned(), "stick".to_owned())]
        )
    }
}
