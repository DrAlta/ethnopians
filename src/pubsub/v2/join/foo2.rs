use std::collections::HashMap;

pub fn foo2<A,B: PartialEq>(working: &mut HashMap<A, B>, a: &Vec<B>) {
    working.retain(|_, v| a.contains(v));
}
