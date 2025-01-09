use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, error::ErrorKind,
    multi::separated_list1, sequence::tuple, IResult,
};

use crate::sandbox::bt::{
    parser::{space_parser, tree_parser},
    Instruction,
};

use super::Thingie;

pub fn sequence_parser<'a, 'b>(
    input: &'a str,
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let mut hash = HashMap::new();
    let (tail, (_, _, _, _, head, _, _)) =
        tuple((
            alt((
                tag("sequence"),
                tag("seq"),
            )),
            space_parser,
            char('{'),
            space_parser,
            separated_list1(
                tuple((
                    space_parser,
                    char(','),
                    space_parser
                )),
                tree_parser
            ),
            space_parser,
            char('}'),
        ))(input)?;

    let mut vec = Vec::new();
    for (idx, thingie) in head.into_iter().enumerate() {
        match thingie {
            Thingie::Token(token) => vec.push(token),
            Thingie::Tree(mut this_i, db) => {
                let thread_name = format!("_{}", idx + 1);
                for (k, mut v) in db.into_iter() {
                    v.correct(&thread_name);
                    assert_eq!(hash.insert(format!("{thread_name}{k}"), v), None,);
                }
                vec.push(thread_name.clone());
                this_i.correct(&thread_name);
                hash.insert(thread_name, this_i);
            }
        }
    }
    Ok((tail, Thingie::Tree(Instruction::Sequence(vec), hash)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_nest_test() {
        let (_, Thingie::Tree(i, db)) =
            sequence_parser("seq{seq{act1, act1}, seq{act2, act2}, act3}").unwrap()
        else {
            panic!()
        };
        assert_eq!(
            i,
            Instruction::Sequence(vec!["_1".to_owned(), "_2".to_owned(), "act3".to_owned()]),
        );
        assert_eq!(
            db,
            HashMap::from([
                (
                    "_1".to_owned(),
                    Instruction::Sequence(vec!["act1".to_owned(), "act1".to_owned()])
                ),
                (
                    "_2".to_owned(),
                    Instruction::Sequence(vec!["act2".to_owned(), "act2".to_owned()])
                ),
            ])
        );
    }
    #[test]
    fn sequence_acts_test() {
        let (_, Thingie::Tree(i, db)) = sequence_parser("seq{act1, act2, act3}").unwrap() else {
            panic!()
        };
        assert_eq!(
            i,
            Instruction::Sequence(vec![
                "act1".to_owned(),
                "act2".to_owned(),
                "act3".to_owned()
            ]),
        );
        assert_eq!(
            db,
            HashMap::new() /*from([
                               ("_2".to_owned(), Instruction::Action(InpulseId::Act1)),
                               ("_3".to_owned(), Instruction::Action(InpulseId::Act2)),
                               ("_4".to_owned(), Instruction::Action(InpulseId::Act3)),
                           ])*/
        );
    }
}
