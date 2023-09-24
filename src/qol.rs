use std::collections::{BTreeMap, HashMap};

use fraction::{GenericFraction, Integer, generic::GenericInteger};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl<T: Clone + Integer + GenericInteger + fraction::FromPrimitive> Sqrt for GenericFraction<T> {
    fn sqrt(self) -> Self {
        let x= format!("{self:.9}").parse::<f32>().unwrap();
        Self::from(x.sqrt())
    }
}

pub trait AddOrInsert<K, V> {
    fn add_or_insert(&mut self, key: K, value:V);
    
}

impl<K: std::hash::Hash + Eq + Ord, V: std::ops::AddAssign<V>> AddOrInsert<K, V> for BTreeMap<K, V> {
    fn add_or_insert(&mut self, key:K, mut value: V) {
        match self.remove(&key) {
            Some(entry) => {
                value += entry;
            }
            None => ()
        }
        self.insert(key, value);
    }
}

impl<K: std::hash::Hash + Eq + Ord, V: std::ops::AddAssign<V>> AddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, key:K, mut value: V) {
        match self.remove(&key) {
            Some(entry) => {
                value += entry;
            }
            None => ()
        }
        self.insert(key, value);
    }
}