use crate::broadphase::sweep_and_prune::Entry;

pub fn sortie(
    Entry {
        aabb: a,
        ..
    }: &Entry,
    Entry {
        aabb: b,
        ..
    }: &Entry,
) -> std::cmp::Ordering {
    a.min_x().total_cmp(&b.min_x())
}
