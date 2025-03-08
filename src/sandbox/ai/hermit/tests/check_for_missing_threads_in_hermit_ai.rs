use std::collections::HashMap;

use qol::{logy, InsertOrInsert};

use crate::sandbox::ai::get_hermit_behavior_task;

#[test]
pub fn check_for_missing_threads_in_hermit_ai() {
    let bt = get_hermit_behavior_task();
    let mut missing = HashMap::new();
    for (thread_name, thread) in &bt {
        for i in thread {
            let x = i.missing_threads_used(&bt);
            if !x.is_empty() {
                missing.insert_or_insert(thread_name, x);
            }
        }
    }
    for (_a, _b) in &missing {
        logy!("log", "\n{_a} is missing:\n{_b:?}\n");
    }
    assert_eq!(missing, HashMap::new())
}
