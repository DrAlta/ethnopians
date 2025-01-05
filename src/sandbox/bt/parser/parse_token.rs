use nom::{
    character::complete::one_of, combinator::recognize, error::ErrorKind, multi::many1, IResult,
};

use super::Thingie;

pub fn parse_token<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, head) = recognize(many1(one_of("abcdefghijklmnopqrstuzwxyx_1234567890")))(input)?;
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
        let (_, Thingie::Token(token)) = parse_token("act1").unwrap() else {
            panic!()
        };
        assert_eq!(token, "act1".to_owned(),);
    }
}
