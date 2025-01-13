//!
//! you want something like 
//! ```
//! pub fn balanced(fill, open, close)
//!     let (tail, body) = open(input)?;
//!     let mut count = 1;
//!     let mut inner_tail = tail;
//!     while {
//!         let(inner_tail ,(filler, term)) = many_till(
//!             fill, 
//!             alt((
//!                 open, 
//!                 close
//!             ))
//!         )(inner_tail)?
//!         if let Ok(_) = open(term) {
//!             count += 1;
//!         } else {
//!             count -=1;
//!             if count == 0 {
//!                 return Ok(inner_tail)
//!             }
//! ```

pub enum Tract<T> {
    Item(T),
    Level(Vec<Tract<T>>),
}
impl<T: Debug> Debug for Tract<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item(arg0) => f.debug_tuple("Item").field(arg0).finish(),
            Self::Level(arg0) => f.debug_tuple("Level").field(arg0).finish(),
        }
    }
}
pub enum InnerTract<T> {
    Item(T),
    Level(Rc<RefCell<Vec<InnerTract<T>>>>),
}


use std::{cell::RefCell, fmt::Debug, rc::Rc};
impl<T: Debug> Debug for InnerTract<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item(arg0) => f.debug_tuple("Item").field(arg0).finish(),
            Self::Level(arg0) => f.debug_tuple("Level").field(arg0).finish(),
        }
    }
}

pub trait Length {
    fn length(&self) -> usize;
}
impl<T> Length for Vec<Tract<T>> {
    fn length(&self) -> usize {
        let mut total = 0;
        for x in self {
            match x {
                Tract::Item(_) => total += 1,
                Tract::Level(vec) => total += vec.length(),
            }
        }
        total
    }
} 

impl<T> From<InnerTract<T>> for Tract<T> {
    fn from(value: InnerTract<T>) -> Self {
        match value {
            InnerTract::Item(x) => Tract::Item(x),
            InnerTract::Level(ref_cell) => {
                let x = Rc::into_inner(ref_cell).unwrap().into_inner().into_iter().map(|x| x.into()).collect();
                Tract::Level(x)
            },
        }
    }
}

use nom::{branch::alt, bytes::complete::tag, character::complete::one_of, error::{ErrorKind, ParseError}, multi::many_till, Err, IResult, InputLength, Parser};
//use nom::{branch::alt, error::{ErrorKind, ParseError}, multi::many_till, Err, IResult, InputLength, Parser};
pub fn balanced<I, O, E, Fill, Open, Close>(
    mut fill: Fill,
    mut open: Open,
    mut close: Close,
) -> impl FnMut(I) -> IResult<I, Vec<Tract<O>>, E> 
where
    I: Clone + InputLength,
    Fill: Parser<I, O, E>,
    Open: Parser<I, I, E>,
    Close: Parser<I, I, E>,
    E: ParseError<I>,
{
    move |input: I| {
        let (mut tail, _body) = open.parse(input)?;
        let building: Rc<RefCell<Vec<InnerTract<O>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut levels = Vec::new();
        levels.push(Rc::clone(&building));
        loop {
            let (inner_tail, (filler, term)) = many_till(
                |x| fill.parse(x), 
                alt((
                    |x| open.parse(x), 
                    |x| close.parse(x)
                ))
            )(tail.clone())?;
            {
                let Some(level_cell) = levels.last() else {
                    return Err(Err::Error(E::from_error_kind(tail, ErrorKind::Fail)));
                };
                let mut level = level_cell.borrow_mut();
                level.extend(filler.into_iter().map(|x| InnerTract::Item(x)));
            }
            if let Ok(_) = open.parse(term) {
    

                let Some(level_cell) = levels.last() else {
                    return Err(Err::Error(E::from_error_kind(tail, ErrorKind::Fail)));
                };
                let x =Rc::clone(level_cell);
                let mut level = x.borrow_mut();


                let new_level = Rc::new(RefCell::new(Vec::new()));
                level.push(
                    InnerTract::Level(
                        Rc::clone(
                            &new_level
                        )
                    )
                );
                levels.push(new_level);
                
                tail = inner_tail
            } else {
                let Some(level_cell) = levels.last() else {
                    return Err(Err::Error(E::from_error_kind(tail, ErrorKind::Fail)));
                };
                let mut level = level_cell.borrow_mut();
                level.pop();
                if levels.is_empty() {
                    return Ok(
                        (
                            inner_tail, 
                            Rc::into_inner(building).unwrap().into_inner().into_iter().map(
                                |x|
                                x.into()
                            ).collect()
                        )
                    )
                }
               /* count -=1;
                if count == 0 {
                    return Ok(inner_tail)
                }
                */
            }
        }
    }
}







fn main(){
    let source = "if yes if no then some then";
    let  (_tail, x)= balanced(
        one_of::<_, _, ()>("if yesnomth"),
        tag("if"),
        tag("then"),
    )(source).unwrap();
    for y in x {
        println!("{y:?}");
    }
}
/*
pub fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    let c = it.next().unwrap_or_default();
                    index += c.len_utf8();
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}
    */