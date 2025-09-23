use crate::{sweep_and_prune::Entry, types::AARect};

pub fn sortie(
    Entry {
        aabb: AARect { min_x: a, .. },
        ..
    }: &Entry,
    Entry {
        aabb: AARect { min_x: b, .. },
        ..
    }: &Entry,
) -> std::cmp::Ordering {
    a.total_cmp(b)
}
