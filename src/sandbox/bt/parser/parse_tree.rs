use nom::{branch::alt, error::ErrorKind, IResult};

use crate::sandbox::bt::parser::parse_selector;

use super::{parse_sequence, parse_token, Thingie};

pub fn parse_tree<'a>(
    input: &'a str,
    //    _prefix: &'b str
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    //    let mut hash = HashMap::new();
    //let x =
    alt((parse_selector, parse_sequence, parse_token))(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::bt::Instruction;

    use super::*;

    #[test]
    fn parse_tree_action_test() {
        let (_, Thingie::Token(token)) = parse_tree("act1").unwrap() else {
            panic!()
        };
        assert_eq!(token, "act1".to_owned());
    }
    #[test]
    fn parse_tree_sel_test() {
        let (_, Thingie::Tree(i, db)) =
            parse_tree("sel{seq{act1, act1}, seq{act2, act2}, act3}").unwrap()
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
                    Instruction::Sequence(vec!["act1".to_owned(), "act1".to_owned()])
                ),
                (
                    "_3".to_owned(),
                    Instruction::Sequence(vec!["act2".to_owned(), "act2".to_owned()])
                ),
            ])
        );
    }
}
