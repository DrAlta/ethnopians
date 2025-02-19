/*
walk through the source table
check if the source item is in the hashmap and if it is then reuse te target
item from the value associated with the source item
*/

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::sandbox::ai::StackItem as Value;

/*
pub trait DeepCopy {
    fn deep_copy(&self) -> Self;
}
*/
trait DeepDeepCopy {
    fn deep_deep_copy(&self, seen: HashMap<ValueRef, Value>) -> Self;
}

#[derive(Debug)]
struct ValueRef(Rc<Value>);
impl Eq for ValueRef {}
impl PartialEq for ValueRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}
impl Hash for ValueRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl ValueRef {
    #[allow(dead_code)]
    fn dup(&self) -> Self {
        ValueRef(Rc::clone(&self.0))
    }
}
impl Value {
    pub fn deep_copy(&self) -> Self {
        let seen: HashMap<ValueRef, Value> = HashMap::new();
        self.deep_deep_copy(seen)
    }
}
impl DeepDeepCopy for Value {
    fn deep_deep_copy(&self, mut seen: HashMap<ValueRef, Value>) -> Self {
        match self {
            Value::Table(rc) => {
                let mut dolly_the_clone = Value::new_table();
                for (key, value) in &*rc.map.borrow_mut() {
                    let key_as_value_ref = ValueRef(Rc::new(key.clone()));
                    if let Some(previous_copy) = seen.get(&key_as_value_ref) {
                        dolly_the_clone
                            .stuff(previous_copy.clone(), key.clone())
                            .expect("Duped value shuldn't have had any parents");
                    } else {
                        dolly_the_clone
                            .stuff(value.clone(), key.clone())
                            .expect("Duped value shuldn't have had any parents");
                        seen.insert(key_as_value_ref, value.clone());
                    }
                }
                dolly_the_clone
            }
            x @ _ => x.clone(),
        }
    }
}
