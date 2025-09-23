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

// This Length trait doesn't appear to be used anywhere and I don't remember why I made it.
#[allow(dead_code)]
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
