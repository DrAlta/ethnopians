use nom::{error::ErrorKind, IResult};

use crate::sandbox::ai::parser::{behavior_tree_parser::Thingie, ident_parser};

pub fn token_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, head) = ident_parser(input)?;
    Ok((tail, Thingie::Token(head.to_owned())))
}

#[cfg(test)]
mod tests {
    use core::panic;
    //use std::collections::HashMap;

    //use crate::sandbox::bt::{InpulseId, Instruction};

    use super::*;

    #[test]
    fn act1_test() {
        let (_, Thingie::Token(token)) = token_parser("act1").unwrap() else {
            panic!()
        };
        assert_eq!(token, "act1".to_owned(),);
    }
}
