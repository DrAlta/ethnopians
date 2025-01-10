use std::collections::HashMap;

use nom::{
    error::ErrorKind, multi::separated_list1, IResult
};

use crate::sandbox::bt::{parser::{space_parser, Thingie}, Corrent, Instruction, Thread, TreePool};

use super::forth_word_parser;


pub fn forth_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, body) = separated_list1(
        space_parser,
        forth_word_parser
    )(input)?;
    let mut thread = Vec::new();
    let mut pool = HashMap::new();
    for (idx, thingie) in body.into_iter().enumerate() {
        match thingie {
            Thingie::Token(token) => {
                thread.push(Instruction::ForthCall(token, 0))
            },
            Thingie::Tree(mut vec, mut hash_map) => {
                let prefix = format!("_{idx}");
                hash_map.correct(&prefix);
                vec.iter_mut().for_each(|x| x.correct(&prefix)); 
                thread.append(&mut vec);
                pool.extend(hash_map.into_iter());
            },
        }
    }
    Ok((
        tail,
        Thingie::Tree(vec![Instruction::Eat(item_a.to_owned())], HashMap::new()),
    ))
}