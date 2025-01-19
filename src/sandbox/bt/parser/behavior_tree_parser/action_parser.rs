use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::tag, combinator::map_res, error::ErrorKind, IResult};

use crate::sandbox::bt::{InpulseId, Instruction};

use super::Thingie;

pub fn action_parser<'a>(input: &'a str) -> IResult<&'a str, Thingie, (&'a str, ErrorKind)> {
    let (tail, i) = alt((
        map_res(tag("act1"), |_| {
            Ok::<Instruction, ()>(Instruction::Action(InpulseId::Act1))
        }),
        map_res(tag("act2"), |_| {
            Ok::<Instruction, ()>(Instruction::Action(InpulseId::Act2))
        }),
        map_res(tag("act3"), |_| {
            Ok::<Instruction, ()>(Instruction::Action(InpulseId::Act3))
        }),
    ))(input)?;
    Ok((tail, Thingie::Tree(vec![i], HashMap::new())))
}
#[cfg(test)]
mod tests {
    use crate::sandbox::bt::InpulseId;

    use super::*;

    #[test]
    fn act1_test() {
        let (_, Thingie::Tree(i, db)) = action_parser("act1").unwrap() else {
            panic!()
        };
        assert_eq!(i, vec![Instruction::Action(InpulseId::Act1)]);
        assert_eq!(db, HashMap::new());
    }
    #[test]
    fn act2_test() {
        let (_, Thingie::Tree(i, db)) = action_parser("act2").unwrap() else {
            panic!()
        };
        assert_eq!(i, vec![Instruction::Action(InpulseId::Act2)]);
        assert_eq!(db, HashMap::new());
    }
    #[test]
    fn act3_test() {
        let (_, Thingie::Tree(i, db)) = action_parser("act3").unwrap() else {
            panic!()
        };
        assert_eq!(i, vec![Instruction::Action(InpulseId::Act3)]);
        assert_eq!(db, HashMap::new());
    }
}
