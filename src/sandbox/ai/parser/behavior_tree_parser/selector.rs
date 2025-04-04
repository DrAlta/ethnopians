use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, error::ErrorKind,
    multi::separated_list1, sequence::tuple, IResult,
};

use crate::sandbox::ai::{
    parser::{behavior_tree_parser::tree_parser, space_parser},
    Instruction, TaskPool,
};

use super::Thingie;

pub fn selector_parser<'a, 'b>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let mut hash = TaskPool::new();
    let (tail, (_, _, _, _, head, _, _)) = //map_res(
        tuple((
            alt((
                tag("selector"),
                tag("sel"),
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
                let thread_name = format!("@{}", idx + 1);
                for (k, mut v) in db.into_iter() {
                    v.iter_mut().for_each(|x| x.correct(&thread_name));
                    assert_eq!(hash.insert(format!("{thread_name}{k}"), v), None,);
                }
                vec.push(thread_name.clone());
                this_i.iter_mut().for_each(|x| x.correct(&thread_name));
                hash.insert(thread_name, this_i);
            }
        }
    }
    Ok((tail, Thingie::Tree(vec![Instruction::Selector(vec)], hash)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_nest_test() {
        let (_, Thingie::Tree(i, db)) =
            selector_parser("sel{sel{act1, act1}, sel{act2, act2}, act3}").unwrap()
        else {
            panic!()
        };
        assert_eq!(
            i,
            vec![Instruction::Selector(vec![
                "@1".to_owned(),
                "@2".to_owned(),
                "act3".to_owned()
            ])],
        );
        assert_eq!(
            db,
            TaskPool::from([
                (
                    "@1".to_owned(),
                    vec![Instruction::Selector(vec![
                        "act1".to_owned(),
                        "act1".to_owned()
                    ])]
                ),
                (
                    "@2".to_owned(),
                    vec![Instruction::Selector(vec![
                        "act2".to_owned(),
                        "act2".to_owned()
                    ])]
                ),
            ])
        );
    }
    #[test]
    fn selector_acts_test() {
        let (_, Thingie::Tree(i, db)) = selector_parser("sel{act1, act2, act3}").unwrap() else {
            panic!()
        };
        assert_eq!(
            i,
            vec![Instruction::Selector(vec![
                "act1".to_owned(),
                "act2".to_owned(),
                "act3".to_owned()
            ])],
        );
        assert_eq!(db, TaskPool::new());
    }
    #[test]
    fn selector_eats_test() {
        let (_, Thingie::Tree(i, db)) =
            selector_parser("sel{eat(pizza), eat(pizza), eat(pizza)}").unwrap()
        else {
            panic!()
        };
        assert_eq!(
            i,
            vec![Instruction::Selector(vec![
                "@1".to_owned(),
                "@2".to_owned(),
                "@3".to_owned()
            ])],
        );
        assert_eq!(
            db,
            TaskPool::from([
                ("@1".to_owned(), vec![Instruction::Eat("pizza".to_owned())]),
                ("@2".to_owned(), vec![Instruction::Eat("pizza".to_owned())]),
                ("@3".to_owned(), vec![Instruction::Eat("pizza".to_owned())]),
            ])
        );
    }
}
