use std::collections::BTreeSet;

/// is `this` more specific than `that`
pub fn is_more_specific<T: Ord>(this: &BTreeSet<T>, that: &BTreeSet<T>) -> bool {
    this.is_superset(that)
}
