use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, error::ErrorKind,
    multi::separated_list1, sequence::tuple, IResult,
};

use crate::sandbox::bt::{
    parser::{parse_space, parse_tree},
    Instruction,
};

use super::Thingie;

pub fn parse_selector<'a, 'b>(
    input: &'a str,
    //    bt: &'b HashMap<ExecutionToken, Vec::<Instruction>>,
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let mut hash = HashMap::new();
    let (tail, (_, _, _, _, head, _, _)) = //map_res(
        tuple((
            alt((
                tag("selector"),
                tag("sel"),
            )),
            parse_space,
            char('{'),
            parse_space,
            separated_list1(
                tuple((
                    parse_space,
                    char(','),
                    parse_space
                )),
                parse_tree
            ),
            parse_space,
            char('}'),
        ))/*,

        |(_, _, head, _, _)| {
            

        }
    )*/
    (input)?;
    let mut vec = Vec::new();
    for (idx, thingie) in head.into_iter().enumerate() {
        match thingie {
            Thingie::Token(token) => vec.push(token),
            Thingie::Tree(i, db) => {
                let thread_name = format!("_{}", idx + 2);
                for (k, mut v) in db.into_iter() {
                    v.correct(&thread_name);
                    assert_eq!(hash.insert(format!("{thread_name}{k}"), v), None,);
                }
                vec.push(thread_name.clone());
                hash.insert(thread_name, i);
            }
        }
    }
    Ok((tail, Thingie::Tree(Instruction::Selector(vec), hash)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_nest_test() {
        let (_, Thingie::Tree(i, db)) =
            parse_selector("sel{sel{act1, act1}, sel{act2, act2}, act3}").unwrap()
        else {
            panic!()
        };
        assert_eq!(
            i,
            Instruction::Selector(vec!["_2".to_owned(), "_3".to_owned(), "act3".to_owned()]),
        );
        assert_eq!(
            db,
            HashMap::from([
                (
                    "_2".to_owned(),
                    Instruction::Selector(vec!["act1".to_owned(), "act1".to_owned()])
                ),
                (
                    "_3".to_owned(),
                    Instruction::Selector(vec!["act2".to_owned(), "act2".to_owned()])
                ),
            ])
        );
    }
    #[test]
    fn selector_acts_test() {
        let (_, Thingie::Tree(i, db)) = parse_selector("sel{act1, act2, act3}").unwrap() else {
            panic!()
        };
        assert_eq!(
            i,
            Instruction::Selector(vec![
                "act1".to_owned(),
                "act2".to_owned(),
                "act3".to_owned()
            ]),
        );
        assert_eq!(db, HashMap::new());
    }
}
