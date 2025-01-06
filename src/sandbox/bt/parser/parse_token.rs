use nom::{
    error::ErrorKind, IResult,
};

use super::{parse_ident, Thingie};

pub fn parse_token<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, head) = parse_ident(input)?;
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
