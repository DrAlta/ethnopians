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
pub fn balanced_parser<I, O, A, B, E, Fill, Open, Close>(
    mut fill: Fill,
    mut open: Open,
    mut close: Close,
) -> impl FnMut(I) -> IResult<I, Vec<Tract<O>>, E>
where
    I: Clone + InputLength,
    Fill: Parser<I, O, E>,
    Open: Parser<I, A, E>,
    Close: Parser<I, B, E>,
    E: ParseError<I>,
{
    move |input: I| {
        let (mut tail, _body) = open.parse(input)?;
        let building: Rc<RefCell<Vec<InnerTract<O>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut levels = Vec::new();
        levels.push(Rc::clone(&building));
        let mut count = 0;
        loop {
            if count == 100 {
                panic!()
            }
            count += 1;

            let (inner_tail, (filler, term)) = many_till::<I, O, bool, _, _, _>(
                |x| fill.parse(x),
                alt((
                    |x| {
                        let (tail, _) = open.parse(x)?;
                        Ok::<(I, bool), Err<_>>((tail, true))
                    },
                    |x| {
                        let (tail, _) = close.parse(x)?;
                        Ok::<(I, bool), Err<_>>((tail, false))
                    },
                )),
            )(tail.clone())?;
            {
                let Some(level_cell) = levels.last() else {
                    return Err(Err::Error(E::from_error_kind(tail, ErrorKind::Fail)));
                };
                let mut level = level_cell.borrow_mut();
                level.extend(filler.into_iter().map(|x| InnerTract::Item(x)));
            }

            if term {
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
                levels.pop();
                if levels.is_empty() {
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
    fn two_test() {
        let source = "ifaifbthencthen";
        let (_tail, x) =
            balanced_parser(one_of::<_, _, ()>("abcdifthen"), tag("if"), tag("then"))(source)
                .unwrap();

        assert_eq!(x, vec![Item('a'), Level(vec![Item('b'),]), Item('c'),])
    }
    #[test]
    fn three_test() {
        /*"if
            a
            if
                b
                if
                    c
                then
                d
            then
            e
        then"*/
        let source = "ifaifbifcthendthenethen";
        let (_tail, x) =
            balanced_parser(one_of::<_, _, ()>("abcdifthen"), tag("if"), tag("then"))(source)
                .unwrap();

        assert_eq!(
            x,
            vec![
                Item('a'),
                Level(vec![Item('b'), Level(vec![Item('c'),]), Item('d'),]),
                Item('e'),
            ]
        )
    }
}
