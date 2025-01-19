use nom::{branch::alt, error::ErrorKind, IResult};

use crate::sandbox::bt::parser::behavior_tree_parser::{combine_parser, eat_parser, forth_tree_parser, inventory_have_ge_parser, sequence_parser,
    token_parser, use_parser, selector_parser, Thingie};


pub fn tree_parser<'a>(
    input: &'a str,
    //    _prefix: &'b str
) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    //    let mut hash = HashMap::new();
    //let x =
    alt((
        combine_parser,
        eat_parser,
        inventory_have_ge_parser,
        selector_parser,
        sequence_parser,
        use_parser,
        forth_tree_parser,
        // token_parser needs to be last so it don't take the prefix of other items
        token_parser,
    ))(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::bt::Instruction;

    use super::*;
    #[test]
    fn parse_debug_test() {
        let (_, Thingie::Tree(_i, _db)) = tree_parser(
            "sel{
        inventory_have_ge(stick, 1), 
        seq{
            go_to_tree,
            use(hands, tree)
        }
    }",
        )
        .unwrap() else {
            panic!()
        };
        // assert_eq!(token, "act1".to_owned());
    }
    #[test]
    fn tree_parser_action_test() {
        let (_, Thingie::Token(token)) = tree_parser("act1").unwrap() else {
            panic!()
        };
        assert_eq!(token, "act1".to_owned());
    }
    #[test]
    fn tree_parser_sel_test() {
        let (_, Thingie::Tree(i, db)) =
            tree_parser("sel{seq{act1, act1}, seq{act2, act2}, act3}").unwrap()
        else {
            panic!()
        };
        assert_eq!(
            i,
            vec![Instruction::Selector(vec![
                "_1".to_owned(),
                "_2".to_owned(),
                "act3".to_owned()
            ])],
        );
        assert_eq!(
            db,
            HashMap::from([
                (
                    "_1".to_owned(),
                    vec![Instruction::Sequence(vec![
                        "act1".to_owned(),
                        "act1".to_owned()
                    ])]
                ),
                (
                    "_2".to_owned(),
                    vec![Instruction::Sequence(vec![
                        "act2".to_owned(),
                        "act2".to_owned()
                    ])]
                ),
            ])
        );
    }
}
