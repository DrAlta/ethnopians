use std::collections::HashMap;

use crate::sandbox::ai::TreePool;

pub trait Corrent {
    fn correct(&mut self, prefix: &str);
}
impl Corrent for TreePool {
    fn correct(&mut self, prefix: &str) {
        let original = std::mem::replace(self, HashMap::new());
        for (k, mut v) in original.into_iter() {
            v.iter_mut().for_each(|x| x.correct(prefix));
            assert_eq!(self.insert(format!("{prefix}{k}"), v), None,);
        }
    }
}
