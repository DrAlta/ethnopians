use nom::{
    branch::alt, error::ErrorKind, IResult
};


use crate::sandbox::bt::parser::{parse_action, parse_selector};

use super::Thingie;


pub fn parse_tree<'a>(
    input: &'a str,
//    _prefix: &'b str
) -> IResult<
    &'a str,
    Thingie,
    (&'a str, ErrorKind),
> {
//    let mut hash = HashMap::new();
    //let x = 
    alt((
        parse_selector,
        parse_action,
    ))(input)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::bt::{InpulseId, Instruction};

    use super::*;

    #[test]
    fn parse_tree_action_test(){
        let (_, Thingie::Tree(i, db)) = parse_action("act1").unwrap() else {
            panic!()
        };
        assert_eq!(
            i,
            Instruction::Action(InpulseId::Act1),
        );
        assert_eq!(
            db,
            HashMap::new()
        );
    }
    
}