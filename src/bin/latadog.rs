use std::rc::Rc;

use ethnolib::stand_alone_complex::latadog::{join_into, Relation, Variable};

/*
trait KeyOn<const INDEX: usize, Key, T>: Sized {
    fn key_on(self)-> Relation<INDEX, Key, T>;
}

impl<Key: Ord, V1> KeyOn<0, Key, (Key, V1)> for Rc<Vec<(Key, V1)>> {
    fn key_on(self) -> Relation<0, Key, (Key, V1)> {
        let mut index: Vec<usize> = (0..self.len()).collect();
        index.sort_unstable_by(|&a,&b| self[a].0.cmp(&self[b].0));
        index.dedup_by(|a,b| self[*a].0 == self[*b].0);
        Relation { elements: self, index, key: PhantomData }
    }
}
*/
fn main() {
    let a = vec![(1_i8, "a"), (3, "C"), (2, "b")];
    let b: Relation<0, _, _> = Rc::new(a).into();
    let input1 = Variable {
        stable: vec![b],
        recent: Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        },
        to_add: vec![Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        }],
    };
    let c = vec![(1_i8, "aa"), (30, "CC"), (20, "bb")];
    let d: Relation<0, _, _> = Rc::new(c).into();
    let input2 = Variable {
        stable: vec![Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        }],
        recent: d,
        to_add: vec![Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        }],
    };
    let mut output = Variable {
        stable: vec![Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        }],
        recent: Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        },
        to_add: vec![Relation {
            elements: Rc::new(Vec::new()),
            index: Vec::new(),
            key: Default::default(),
        }],
    };
    join_into(&input1, &input2, &mut output, |k, _, t2| {
        println!("foo");
        (*k, t2.1)
    });
    println!("{output:?}");
}
