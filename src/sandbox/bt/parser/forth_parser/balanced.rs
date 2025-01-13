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

impl<T: PartialEq> PartialEq for Tract<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Item(l0), Self::Item(r0)) => l0 == r0,
            (Self::Level(l0), Self::Level(r0)) => l0 == r0,
            _ => false,
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
                let x = Rc::into_inner(ref_cell)
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|x| x.into())
                    .collect();
                Tract::Level(x)
            }
        }
    }
}

use nom::{
    branch::alt,
    error::{ErrorKind, ParseError},
    multi::many_till,
    Err, IResult, InputLength, Parser,
};
pub fn balanced<I, O, E, Fill, Open, Close>(
    mut fill: Fill,
    mut open: Open,
    mut close: Close,
) -> impl FnMut(I) -> IResult<I, Vec<Tract<O>>, E>
where
    I: Clone + InputLength + Debug,
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
        let mut count = 0;
        loop {
            println!("{count}:{tail:?}");
            if count == 10 {
                panic!()
            }
            count += 1;
            let (inner_tail, (filler, term)) = many_till(
                |x| fill.parse(x),
                alt((|x| open.parse(x), |x| close.parse(x))),
            )(tail.clone())?;
            {
                let Some(level_cell) = levels.last() else {
                    return Err(Err::Error(E::from_error_kind(tail, ErrorKind::Fail)));
                };
                let mut level = level_cell.borrow_mut();
                level.extend(filler.into_iter().map(|x| InnerTract::Item(x)));
            }
            if let Ok(_) = open.parse(term) {
                println!("open");

                let Some(level_cell) = levels.last() else {
                    return Err(Err::Error(E::from_error_kind(tail, ErrorKind::Fail)));
                };
                let x = Rc::clone(level_cell);
                let mut level = x.borrow_mut();

                let new_level = Rc::new(RefCell::new(Vec::new()));
                level.push(InnerTract::Level(Rc::clone(&new_level)));
                levels.push(new_level);

                tail = inner_tail
            } else {
                println!("close");
                levels.pop();
                if levels.is_empty() {
                    println!("returning ok");
                    return Ok((
                        inner_tail,
                        Rc::into_inner(building)
                            .unwrap()
                            .into_inner()
                            .into_iter()
                            .map(|x| x.into())
                            .collect(),
                    ));
                } else {
                    tail = inner_tail;
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

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::tag, character::complete::one_of};

    use super::*;
    use Tract::*;

    #[test]
    fn main() {
        let source = "if yes if no then some then";
        let (_tail, x) =
            balanced(one_of::<_, _, ()>("if yesnomth"), tag("if"), tag("then"))(source).unwrap();

        assert_eq!(
            x,
            vec![
                Item(' '),
                Item('y'),
                Item('e'),
                Item('s'),
                Item(' '),
                Level(vec![Item(' '), Item('n'), Item('o'), Item(' '),]),
                Item(' '),
                Item('s'),
                Item('o'),
                Item('m'),
                Item('e'),
                Item(' '),
            ]
        )
    }
}
